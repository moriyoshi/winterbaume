use chrono::{DateTime, Utc};

/// An EFS file system.
#[derive(Debug, Clone)]
pub struct FileSystem {
    pub file_system_id: String,
    pub file_system_arn: String,
    pub creation_time: DateTime<Utc>,
    pub owner_id: String,
    pub creation_token: String,
    pub performance_mode: String,
    pub throughput_mode: String,
    pub life_cycle_state: String,
    pub number_of_mount_targets: i32,
    pub size_in_bytes: FileSizeValue,
    pub encrypted: bool,
    pub tags: Vec<Tag>,
    pub name: Option<String>,
    pub policy: Option<String>,
    pub lifecycle_policies: Vec<LifecyclePolicy>,
    pub backup_policy_status: Option<String>,
}

/// File system size value.
#[derive(Debug, Clone)]
pub struct FileSizeValue {
    pub value: i64,
    pub timestamp: Option<DateTime<Utc>>,
    pub value_in_ia: i64,
    pub value_in_standard: i64,
}

/// Lifecycle policy.
#[derive(Debug, Clone)]
pub struct LifecyclePolicy {
    pub transition_to_ia: Option<String>,
    pub transition_to_primary_storage_class: Option<String>,
    pub transition_to_archive: Option<String>,
}

/// An EFS mount target.
#[derive(Debug, Clone)]
pub struct MountTarget {
    pub mount_target_id: String,
    pub file_system_id: String,
    pub subnet_id: String,
    pub life_cycle_state: String,
    pub ip_address: String,
    pub network_interface_id: String,
    pub availability_zone_id: String,
    pub availability_zone_name: String,
    pub owner_id: String,
    pub security_groups: Vec<String>,
}

/// An EFS access point.
#[derive(Debug, Clone)]
pub struct AccessPoint {
    pub access_point_id: String,
    pub access_point_arn: String,
    pub client_token: String,
    pub file_system_id: String,
    pub life_cycle_state: String,
    pub name: Option<String>,
    pub owner_id: String,
    pub posix_user: Option<PosixUser>,
    pub root_directory: Option<RootDirectory>,
    pub tags: Vec<Tag>,
}

/// POSIX user identity for an access point.
#[derive(Debug, Clone)]
pub struct PosixUser {
    pub uid: i64,
    pub gid: i64,
    pub secondary_gids: Option<Vec<i64>>,
}

/// Root directory for an access point.
#[derive(Debug, Clone)]
pub struct RootDirectory {
    pub path: Option<String>,
    pub creation_info: Option<CreationInfo>,
}

/// Creation info for the root directory of an access point.
#[derive(Debug, Clone)]
pub struct CreationInfo {
    pub owner_uid: i64,
    pub owner_gid: i64,
    pub permissions: String,
}

/// A key-value tag.
#[derive(Debug, Clone)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

/// A replication destination.
#[derive(Debug, Clone)]
pub struct ReplicationDestination {
    pub file_system_id: String,
    pub region: String,
    pub status: String,
    pub role_arn: Option<String>,
}

/// A replication configuration.
#[derive(Debug, Clone)]
pub struct ReplicationConfiguration {
    pub source_file_system_id: String,
    pub source_file_system_arn: String,
    pub original_source_file_system_arn: String,
    pub source_file_system_region: String,
    pub source_file_system_owner_id: String,
    pub creation_time: chrono::DateTime<chrono::Utc>,
    pub destinations: Vec<ReplicationDestination>,
}

/// Account resource ID preferences.
#[derive(Debug, Clone)]
pub struct AccountPreferences {
    pub resource_id_type: String,
    pub resources: Vec<String>,
}

/// Per-filesystem protection settings.
#[derive(Debug, Clone)]
pub struct FileSystemProtection {
    pub replication_overwrite_protection: Option<String>,
}

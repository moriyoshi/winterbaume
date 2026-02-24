use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Cluster {
    pub cluster_id: String,
    pub hsm_type: String,
    pub subnet_mapping: HashMap<String, String>,
    pub vpc_id: String,
    pub state: ClusterState,
    pub security_group: String,
    pub source_backup_id: Option<String>,
    pub backup_policy: String,
    pub backup_retention_policy: Option<BackupRetentionPolicy>,
    pub create_timestamp: f64,
    pub tag_list: Vec<Tag>,
    pub region: String,
    pub account_id: String,
    pub hsms: Vec<Hsm>,
}

#[derive(Debug, Clone)]
pub struct Hsm {
    pub hsm_id: String,
    pub cluster_id: String,
    pub availability_zone: String,
    pub subnet_id: Option<String>,
    pub eni_id: Option<String>,
    pub eni_ip: Option<String>,
    pub state: String,
}

#[derive(Debug, Clone)]
pub struct Backup {
    pub backup_id: String,
    pub backup_arn: String,
    pub backup_state: String,
    pub cluster_id: Option<String>,
    pub create_timestamp: f64,
    pub copy_timestamp: Option<f64>,
    pub delete_timestamp: Option<f64>,
    pub hsm_type: Option<String>,
    pub never_expires: bool,
    pub source_backup: Option<String>,
    pub source_cluster: Option<String>,
    pub source_region: Option<String>,
    pub tag_list: Vec<Tag>,
}

#[derive(Debug, Clone)]
pub struct BackupRetentionPolicy {
    pub r#type: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClusterState {
    CreateInProgress,
    Uninitialized,
    InitializeInProgress,
    Initialized,
    Active,
    DeleteInProgress,
    Deleted,
}

impl ClusterState {
    pub fn as_str(&self) -> &str {
        match self {
            ClusterState::CreateInProgress => "CREATE_IN_PROGRESS",
            ClusterState::Uninitialized => "UNINITIALIZED",
            ClusterState::InitializeInProgress => "INITIALIZE_IN_PROGRESS",
            ClusterState::Initialized => "INITIALIZED",
            ClusterState::Active => "ACTIVE",
            ClusterState::DeleteInProgress => "DELETE_IN_PROGRESS",
            ClusterState::Deleted => "DELETED",
        }
    }
}

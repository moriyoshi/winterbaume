use chrono::{DateTime, Utc};

/// Represents an AWS Directory Service directory.
#[derive(Debug, Clone)]
pub struct Directory {
    pub directory_id: String,
    pub name: String,
    pub short_name: Option<String>,
    pub description: Option<String>,
    pub size: String,
    pub directory_type: DirectoryType,
    pub alias: String,
    pub access_url: String,
    pub stage: DirectoryStage,
    pub launch_time: DateTime<Utc>,
    pub stage_last_updated_date_time: DateTime<Utc>,
    pub dns_ip_addrs: Vec<String>,
    pub vpc_settings: Option<DirectoryVpcSettings>,
    pub connect_settings: Option<DirectoryConnectSettings>,
    pub ssoid_enabled: bool,
}

/// Type of directory.
#[derive(Debug, Clone, PartialEq)]
pub enum DirectoryType {
    SimpleAD,
    ADConnector,
    MicrosoftAD,
}

impl DirectoryType {
    pub fn as_str(&self) -> &str {
        match self {
            DirectoryType::SimpleAD => "SimpleAD",
            DirectoryType::ADConnector => "ADConnector",
            DirectoryType::MicrosoftAD => "MicrosoftAD",
        }
    }
}

/// Stage (status) of a directory.
#[derive(Debug, Clone, PartialEq)]
pub enum DirectoryStage {
    Requested,
    Creating,
    Created,
    Active,
    Inoperable,
    Impaired,
    Restoring,
    RestoreFailed,
    Deleting,
    Deleted,
    Failed,
}

impl DirectoryStage {
    pub fn as_str(&self) -> &str {
        match self {
            DirectoryStage::Requested => "Requested",
            DirectoryStage::Creating => "Creating",
            DirectoryStage::Created => "Created",
            DirectoryStage::Active => "Active",
            DirectoryStage::Inoperable => "Inoperable",
            DirectoryStage::Impaired => "Impaired",
            DirectoryStage::Restoring => "Restoring",
            DirectoryStage::RestoreFailed => "RestoreFailed",
            DirectoryStage::Deleting => "Deleting",
            DirectoryStage::Deleted => "Deleted",
            DirectoryStage::Failed => "Failed",
        }
    }
}

/// VPC settings for a directory.
#[derive(Debug, Clone)]
pub struct DirectoryVpcSettings {
    pub vpc_id: String,
    pub subnet_ids: Vec<String>,
    pub security_group_id: String,
    pub availability_zones: Vec<String>,
}

/// Connect settings for an AD Connector directory.
#[derive(Debug, Clone)]
pub struct DirectoryConnectSettings {
    pub vpc_id: String,
    pub subnet_ids: Vec<String>,
    pub customer_dns_ips: Vec<String>,
    pub customer_user_name: String,
    pub security_group_id: String,
    pub availability_zones: Vec<String>,
    pub connect_ips: Vec<String>,
}

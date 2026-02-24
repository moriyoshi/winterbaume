use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Vault {
    pub vault_name: String,
    pub arn: String,
    pub created_at: DateTime<Utc>,
    pub archives: HashMap<String, Archive>,
    pub jobs: HashMap<String, Job>,
    pub tags: HashMap<String, String>,
    pub access_policy: Option<String>,
    pub notification_config: Option<VaultNotificationConfig>,
    pub vault_lock: Option<VaultLock>,
    pub multipart_uploads: HashMap<String, MultipartUpload>,
}

#[derive(Debug, Clone)]
pub struct VaultNotificationConfig {
    pub sns_topic: Option<String>,
    pub events: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct VaultLock {
    pub lock_id: String,
    pub policy: Option<String>,
    pub state: VaultLockState,
    pub creation_date: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VaultLockState {
    InProgress,
    Locked,
}

#[derive(Debug, Clone)]
pub struct MultipartUpload {
    pub upload_id: String,
    pub vault_arn: String,
    pub description: Option<String>,
    pub part_size: Option<i64>,
    pub creation_date: DateTime<Utc>,
    pub parts: HashMap<String, MultipartPart>,
}

#[derive(Debug, Clone)]
pub struct MultipartPart {
    pub range_in_bytes: String,
    pub sha256: String,
}

#[derive(Debug, Clone)]
pub struct DataRetrievalPolicy {
    pub rules: Vec<DataRetrievalRule>,
}

#[derive(Debug, Clone)]
pub struct DataRetrievalRule {
    pub bytes_per_hour: Option<i64>,
    pub strategy: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ProvisionedCapacity {
    pub capacity_id: String,
    pub start_date: DateTime<Utc>,
    pub expiration_date: DateTime<Utc>,
}

/// A Glacier archive.
#[derive(Debug, Clone)]
pub struct Archive {
    pub archive_id: String,
    /// Key used to retrieve the archive body from the service's [`BlobStore`].
    pub blob_key: String,
    /// Size of the archive body in bytes.
    pub size: usize,
    pub sha256: String,
    pub description: String,
    pub creation_date: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Job {
    pub job_id: String,
    pub job_type: JobType,
    pub tier: String,
    pub vault_arn: String,
    pub archive_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum JobType {
    ArchiveRetrieval,
    InventoryRetrieval,
}

impl Job {
    pub fn is_completed(&self) -> bool {
        Utc::now() > self.completed_at
    }
}

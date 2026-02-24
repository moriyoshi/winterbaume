use std::collections::HashMap;

/// A snapshot managed by EBS.
#[derive(Debug, Clone)]
pub struct Snapshot {
    pub snapshot_id: String,
    pub volume_size: i64,
    pub block_size: i32,
    pub description: String,
    pub status: SnapshotStatus,
    /// Map from block index to block metadata (body lives in the BlobStore).
    pub blocks: HashMap<i32, Block>,
}

/// Status of a snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnapshotStatus {
    Pending,
    Completed,
    Error,
}

impl SnapshotStatus {
    pub fn as_str(&self) -> &str {
        match self {
            SnapshotStatus::Pending => "pending",
            SnapshotStatus::Completed => "completed",
            SnapshotStatus::Error => "error",
        }
    }
}

/// An EBS volume.
#[derive(Debug, Clone)]
pub struct Volume {
    pub volume_id: String,
    pub availability_zone: String,
    pub size: i64,
    pub volume_type: String,
    pub iops: Option<i64>,
    pub throughput: Option<i64>,
    pub encrypted: bool,
    pub kms_key_id: Option<String>,
    pub snapshot_id: Option<String>,
    pub state: String,
    pub tags: HashMap<String, String>,
}

/// A block within a snapshot.
#[derive(Debug, Clone)]
pub struct Block {
    pub block_index: i32,
    pub block_token: String,
    /// Key used to retrieve the block body from the service's [`BlobStore`].
    pub blob_key: String,
    /// Size of the block body in bytes.
    pub content_length: u64,
    pub checksum: String,
    pub checksum_algorithm: String,
}

//! Serde-compatible view types for EBS state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{BlobBackedService, StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::EbsService;
use crate::state::EbsState;
use crate::types::{Block, Snapshot, SnapshotStatus, Volume};

/// Serializable view of the entire EBS state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EbsStateView {
    /// Snapshots keyed by snapshot ID.
    #[serde(default)]
    pub snapshots: HashMap<String, SnapshotView>,
    /// Volumes keyed by volume ID.
    #[serde(default)]
    pub volumes: HashMap<String, VolumeView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotView {
    pub snapshot_id: String,
    pub volume_size: i64,
    pub block_size: i32,
    pub description: String,
    /// Snapshot status as string: "pending", "completed", "error"
    pub status: String,
    /// Blocks keyed by block index (as string for JSON compatibility).
    #[serde(default)]
    pub blocks: HashMap<i32, BlockView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockView {
    pub block_index: i32,
    pub block_token: String,
    pub blob_key: String,
    pub content_length: u64,
    pub checksum: String,
    pub checksum_algorithm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeView {
    pub volume_id: String,
    pub availability_zone: String,
    pub size: i64,
    pub volume_type: String,
    #[serde(default)]
    pub iops: Option<i64>,
    #[serde(default)]
    pub throughput: Option<i64>,
    pub encrypted: bool,
    #[serde(default)]
    pub kms_key_id: Option<String>,
    #[serde(default)]
    pub snapshot_id: Option<String>,
    pub state: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

fn parse_snapshot_status(s: &str) -> SnapshotStatus {
    match s {
        "completed" => SnapshotStatus::Completed,
        "error" => SnapshotStatus::Error,
        _ => SnapshotStatus::Pending,
    }
}

// --- From internal types to view types ---

impl From<&EbsState> for EbsStateView {
    fn from(state: &EbsState) -> Self {
        EbsStateView {
            snapshots: state
                .snapshots
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        SnapshotView {
                            snapshot_id: v.snapshot_id.clone(),
                            volume_size: v.volume_size,
                            block_size: v.block_size,
                            description: v.description.clone(),
                            status: v.status.as_str().to_string(),
                            blocks: v
                                .blocks
                                .iter()
                                .map(|(bi, b)| {
                                    (
                                        *bi,
                                        BlockView {
                                            block_index: b.block_index,
                                            block_token: b.block_token.clone(),
                                            blob_key: b.blob_key.clone(),
                                            content_length: b.content_length,
                                            checksum: b.checksum.clone(),
                                            checksum_algorithm: b.checksum_algorithm.clone(),
                                        },
                                    )
                                })
                                .collect(),
                        },
                    )
                })
                .collect(),
            volumes: state
                .volumes
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        VolumeView {
                            volume_id: v.volume_id.clone(),
                            availability_zone: v.availability_zone.clone(),
                            size: v.size,
                            volume_type: v.volume_type.clone(),
                            iops: v.iops,
                            throughput: v.throughput,
                            encrypted: v.encrypted,
                            kms_key_id: v.kms_key_id.clone(),
                            snapshot_id: v.snapshot_id.clone(),
                            state: v.state.clone(),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<EbsStateView> for EbsState {
    fn from(view: EbsStateView) -> Self {
        EbsState {
            snapshots: view
                .snapshots
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Snapshot {
                            snapshot_id: v.snapshot_id,
                            volume_size: v.volume_size,
                            block_size: v.block_size,
                            description: v.description,
                            status: parse_snapshot_status(&v.status),
                            blocks: v
                                .blocks
                                .into_iter()
                                .map(|(bi, b)| {
                                    (
                                        bi,
                                        Block {
                                            block_index: b.block_index,
                                            block_token: b.block_token,
                                            blob_key: b.blob_key,
                                            content_length: b.content_length,
                                            checksum: b.checksum,
                                            checksum_algorithm: b.checksum_algorithm,
                                        },
                                    )
                                })
                                .collect(),
                        },
                    )
                })
                .collect(),
            volumes: view
                .volumes
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Volume {
                            volume_id: v.volume_id,
                            availability_zone: v.availability_zone,
                            size: v.size,
                            volume_type: v.volume_type,
                            iops: v.iops,
                            throughput: v.throughput,
                            encrypted: v.encrypted,
                            kms_key_id: v.kms_key_id,
                            snapshot_id: v.snapshot_id,
                            state: v.state,
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- Shared snapshot logic ---

impl EbsService {
    fn snapshot_inner(state: &EbsState) -> (EbsStateView, Vec<String>) {
        let view = EbsStateView::from(state);
        let keys = view
            .snapshots
            .values()
            .flat_map(|s| s.blocks.values())
            .filter(|b| !b.blob_key.is_empty())
            .map(|b| b.blob_key.clone())
            .collect();
        (view, keys)
    }
}

// --- StatefulService implementation ---

impl StatefulService for EbsService {
    type StateView = EbsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        Self::snapshot_inner(&guard).0
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = EbsState::from(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            let incoming = EbsState::from(view);
            for (k, v) in incoming.snapshots {
                guard.snapshots.insert(k, v);
            }
            for (k, v) in incoming.volumes {
                guard.volumes.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

impl BlobBackedService for EbsService {
    async fn snapshot_with_blobs(
        &self,
        account_id: &str,
        region: &str,
        visitor: &mut dyn winterbaume_core::BlobVisitor,
    ) -> Result<EbsStateView, winterbaume_core::StateViewError> {
        let blobs = self.blobs.get(account_id, region);
        let state = self.state.get(account_id, region);
        let lock = state.read().await;
        let (view, keys) = Self::snapshot_inner(&lock);
        for chunk in keys.chunks(winterbaume_core::DEFAULT_BLOB_BATCH_SIZE) {
            let mut entries = Vec::with_capacity(chunk.len());
            for key in chunk {
                let size = blobs
                    .stat(key)
                    .await
                    .map_err(winterbaume_core::StateViewError::Blob)?
                    .map(|s| s.size);
                let reader = blobs
                    .get_reader(key)
                    .await
                    .map_err(winterbaume_core::StateViewError::Blob)?;
                entries.push(winterbaume_core::BlobExportEntry {
                    key: key.clone(),
                    reader,
                    size,
                });
            }
            visitor
                .visit(entries)
                .await
                .map_err(winterbaume_core::StateViewError::Blob)?;
        }
        Ok(view)
    }

    async fn restore_with_blobs(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
        source: &mut dyn winterbaume_core::BlobSource,
    ) -> Result<(), winterbaume_core::StateViewError> {
        let blobs = self.blobs.get(account_id, region);
        let new_state = EbsState::from(view);
        // Import blobs referenced by the incoming view before replacing state.
        for snapshot in new_state.snapshots.values() {
            for block in snapshot.blocks.values() {
                if !block.blob_key.is_empty() {
                    let mut reader = source.fetch(block.blob_key.clone()).await?;
                    blobs
                        .put_from(&block.blob_key, &mut reader)
                        .await
                        .map_err(winterbaume_core::StateViewError::Blob)?;
                }
            }
        }
        let state = self.state.get(account_id, region);
        let mut guard = state.write().await;
        *guard = new_state;
        Ok(())
    }
}

use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct EbsState {
    pub snapshots: HashMap<String, Snapshot>,
    pub volumes: HashMap<String, Volume>,
}

#[derive(Debug, Error)]
pub enum EbsError {
    #[error("VolumeSize must be greater than 0")]
    InvalidVolumeSize,

    #[error("Snapshot {snapshot_id} not found")]
    SnapshotNotFound { snapshot_id: String },

    #[error("Snapshot {snapshot_id} is not in pending state")]
    SnapshotNotPending { snapshot_id: String },
}

impl EbsState {
    pub fn start_snapshot(
        &mut self,
        volume_size: i64,
        description: &str,
    ) -> Result<&Snapshot, EbsError> {
        if volume_size <= 0 {
            return Err(EbsError::InvalidVolumeSize);
        }

        let snapshot_id = format!("snap-{}", uuid::Uuid::new_v4().simple());
        // Truncate to match AWS format: snap- + 17 hex chars
        let snapshot_id = snapshot_id[..22].to_string();

        let snapshot = Snapshot {
            snapshot_id: snapshot_id.clone(),
            volume_size,
            block_size: 524288, // 512 KiB, standard EBS block size
            description: description.to_string(),
            status: SnapshotStatus::Pending,
            blocks: HashMap::new(),
        };

        self.snapshots.insert(snapshot_id.clone(), snapshot);
        Ok(self.snapshots.get(&snapshot_id).unwrap())
    }

    pub fn complete_snapshot(
        &mut self,
        snapshot_id: &str,
        changed_blocks_count: i32,
    ) -> Result<&Snapshot, EbsError> {
        let snapshot =
            self.snapshots
                .get_mut(snapshot_id)
                .ok_or_else(|| EbsError::SnapshotNotFound {
                    snapshot_id: snapshot_id.to_string(),
                })?;

        if snapshot.status != SnapshotStatus::Pending {
            return Err(EbsError::SnapshotNotPending {
                snapshot_id: snapshot_id.to_string(),
            });
        }

        let _ = changed_blocks_count; // We accept it but don't validate block count
        snapshot.status = SnapshotStatus::Completed;
        Ok(self.snapshots.get(snapshot_id).unwrap())
    }

    /// Store a block with a pre-computed blob key and content length.
    ///
    /// The caller is responsible for writing the block data to the BlobStore
    /// under `blob_key` before calling this method.
    pub fn put_snapshot_block(
        &mut self,
        snapshot_id: &str,
        block_index: i32,
        blob_key: String,
        content_length: u64,
        checksum: &str,
        checksum_algorithm: &str,
    ) -> Result<(&str, &str), EbsError> {
        let snapshot =
            self.snapshots
                .get_mut(snapshot_id)
                .ok_or_else(|| EbsError::SnapshotNotFound {
                    snapshot_id: snapshot_id.to_string(),
                })?;

        if snapshot.status != SnapshotStatus::Pending {
            return Err(EbsError::SnapshotNotPending {
                snapshot_id: snapshot_id.to_string(),
            });
        }

        let block_token = format!("token-{}-{}", snapshot_id, block_index);
        let block = Block {
            block_index,
            block_token,
            blob_key,
            content_length,
            checksum: checksum.to_string(),
            checksum_algorithm: checksum_algorithm.to_string(),
        };

        snapshot.blocks.insert(block_index, block);
        let block = snapshot.blocks.get(&block_index).unwrap();
        Ok((&block.checksum, &block.checksum_algorithm))
    }

    pub fn list_snapshot_blocks(
        &self,
        snapshot_id: &str,
        max_results: Option<i32>,
        starting_block_index: Option<i32>,
    ) -> Result<(Vec<&Block>, i64, i32), EbsError> {
        let snapshot =
            self.snapshots
                .get(snapshot_id)
                .ok_or_else(|| EbsError::SnapshotNotFound {
                    snapshot_id: snapshot_id.to_string(),
                })?;

        let start = starting_block_index.unwrap_or(0);
        let limit = max_results.unwrap_or(100) as usize;

        let mut blocks: Vec<&Block> = snapshot
            .blocks
            .values()
            .filter(|b| b.block_index >= start)
            .collect();
        blocks.sort_by_key(|b| b.block_index);
        blocks.truncate(limit);

        Ok((blocks, snapshot.volume_size, snapshot.block_size))
    }
}

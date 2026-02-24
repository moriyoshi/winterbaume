//! At-rest serialisation types for Glacier state persistence.
//!
//! Each struct is the on-disk JSON representation of one piece of Glacier state,
//! written to the BlobStore whenever the corresponding in-memory state mutates.
//! Together they allow the full [`crate::state::GlacierState`] to be
//! reconstructed from the BlobStore alone, without a separate JSON snapshot.
//!
//! # Storage layout (relative to BlobStore namespace)
//!
//! ```text
//! meta/bucket-config/{vault_name}                            — StoredVaultConfig
//! meta/version-meta/{vault_name}/{archive_id}                — StoredArchiveMeta
//! ```

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{Archive, Vault};

// ---------------------------------------------------------------------------
// Vault configuration
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredVaultConfig {
    pub vault_name: String,
    pub arn: String,
    pub created_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
    pub access_policy: Option<String>,
    pub notification_config: Option<StoredNotificationConfig>,
    pub vault_lock: Option<StoredVaultLock>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredNotificationConfig {
    pub sns_topic: Option<String>,
    pub events: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredVaultLock {
    pub lock_id: String,
    pub policy: Option<String>,
    pub state: String,
    pub creation_date: DateTime<Utc>,
}

impl StoredVaultConfig {
    pub fn from_vault(vault: &Vault) -> Self {
        Self {
            vault_name: vault.vault_name.clone(),
            arn: vault.arn.clone(),
            created_at: vault.created_at,
            tags: vault.tags.clone(),
            access_policy: vault.access_policy.clone(),
            notification_config: vault.notification_config.as_ref().map(|c| {
                StoredNotificationConfig {
                    sns_topic: c.sns_topic.clone(),
                    events: c.events.clone(),
                }
            }),
            vault_lock: vault.vault_lock.as_ref().map(|l| StoredVaultLock {
                lock_id: l.lock_id.clone(),
                policy: l.policy.clone(),
                state: match l.state {
                    crate::types::VaultLockState::InProgress => "InProgress".to_string(),
                    crate::types::VaultLockState::Locked => "Locked".to_string(),
                },
                creation_date: l.creation_date,
            }),
        }
    }

    pub fn into_vault(self) -> Vault {
        Vault {
            vault_name: self.vault_name,
            arn: self.arn,
            created_at: self.created_at,
            archives: HashMap::new(),
            jobs: HashMap::new(),
            tags: self.tags,
            access_policy: self.access_policy,
            notification_config: self.notification_config.map(|c| {
                crate::types::VaultNotificationConfig {
                    sns_topic: c.sns_topic,
                    events: c.events,
                }
            }),
            vault_lock: self.vault_lock.map(|l| crate::types::VaultLock {
                lock_id: l.lock_id,
                policy: l.policy,
                state: if l.state == "Locked" {
                    crate::types::VaultLockState::Locked
                } else {
                    crate::types::VaultLockState::InProgress
                },
                creation_date: l.creation_date,
            }),
            multipart_uploads: HashMap::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Archive metadata
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredArchiveMeta {
    pub vault_name: String,
    pub archive_id: String,
    pub blob_key: String,
    pub size: usize,
    pub sha256: String,
    pub description: String,
    pub creation_date: DateTime<Utc>,
}

impl StoredArchiveMeta {
    pub fn from_archive(vault_name: &str, archive: &Archive) -> Self {
        Self {
            vault_name: vault_name.to_string(),
            archive_id: archive.archive_id.clone(),
            blob_key: archive.blob_key.clone(),
            size: archive.size,
            sha256: archive.sha256.clone(),
            description: archive.description.clone(),
            creation_date: archive.creation_date,
        }
    }

    pub fn into_archive(self) -> Archive {
        Archive {
            archive_id: self.archive_id,
            blob_key: self.blob_key,
            size: self.size,
            sha256: self.sha256,
            description: self.description,
            creation_date: self.creation_date,
        }
    }
}

//! At-rest serialisation types for S3 state persistence.
//!
//! Each struct is the on-disk JSON representation of one piece of S3 state,
//! written to the BlobStore whenever the corresponding in-memory state mutates.
//! Together they allow the full [`crate::state::S3State`] to be reconstructed
//! from the BlobStore alone, without a separate JSON snapshot.
//!
//! # Storage layout (relative to BlobStore namespace)
//!
//! ```text
//! meta/version-meta/{blob_key}/{blob_version_id}   — StoredObjectVersion
//! meta/delete-markers/{blob_key}/{dm_version_id}   — StoredDeleteMarker
//! meta/bucket-config/{bucket_name}                 — StoredBucketConfig
//! ```

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::state::{BucketState, PublicAccessBlockConfig};
use crate::types::{DeleteMarker, Object};

// ---------------------------------------------------------------------------
// Object version
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredObjectVersion {
    pub bucket: String,
    pub key: String,
    pub blob_key: String,
    pub version_id: String,
    pub blob_version_id: String,
    pub etag: String,
    pub content_type: String,
    pub content_length: u64,
    pub last_modified: DateTime<Utc>,
    pub storage_class: String,
    pub metadata: Vec<(String, String)>,
    pub tags: HashMap<String, String>,
    pub acl: Option<String>,
    pub legal_hold_status: Option<String>,
    pub retention_mode: Option<String>,
    pub retain_until_date: Option<String>,
}

impl StoredObjectVersion {
    pub fn from_object(bucket: &str, obj: &Object) -> Self {
        Self {
            bucket: bucket.to_string(),
            key: obj.key.clone(),
            blob_key: obj.blob_key.clone(),
            version_id: obj.version_id.clone(),
            blob_version_id: obj.blob_version_id.clone(),
            etag: obj.etag.clone(),
            content_type: obj.content_type.clone(),
            content_length: obj.content_length,
            last_modified: obj.last_modified,
            storage_class: obj.storage_class.clone(),
            metadata: obj.metadata.clone(),
            tags: obj.tags.clone(),
            acl: obj.acl.clone(),
            legal_hold_status: obj.legal_hold_status.clone(),
            retention_mode: obj.retention_mode.clone(),
            retain_until_date: obj.retain_until_date.clone(),
        }
    }

    pub fn into_object(self) -> Object {
        Object {
            key: self.key,
            blob_key: self.blob_key,
            version_id: self.version_id,
            blob_version_id: self.blob_version_id,
            etag: self.etag,
            content_type: self.content_type,
            content_length: self.content_length,
            last_modified: self.last_modified,
            storage_class: self.storage_class,
            metadata: self.metadata,
            tags: self.tags,
            acl: self.acl,
            legal_hold_status: self.legal_hold_status,
            retention_mode: self.retention_mode,
            retain_until_date: self.retain_until_date,
        }
    }
}

// ---------------------------------------------------------------------------
// Delete marker
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredDeleteMarker {
    pub bucket: String,
    pub key: String,
    pub blob_key: String,
    pub version_id: String,
    pub last_modified: DateTime<Utc>,
}

impl StoredDeleteMarker {
    pub fn from_marker(bucket: &str, blob_key: &str, dm: &DeleteMarker) -> Self {
        Self {
            bucket: bucket.to_string(),
            key: dm.key.clone(),
            blob_key: blob_key.to_string(),
            version_id: dm.version_id.clone(),
            last_modified: dm.last_modified,
        }
    }

    pub fn into_marker(self) -> DeleteMarker {
        DeleteMarker {
            key: self.key,
            version_id: self.version_id,
            last_modified: self.last_modified,
        }
    }
}

// ---------------------------------------------------------------------------
// Bucket configuration
// ---------------------------------------------------------------------------

/// At-rest form of [`PublicAccessBlockConfig`], which does not derive serde.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredPublicAccessBlock {
    pub block_public_acls: bool,
    pub ignore_public_acls: bool,
    pub block_public_policy: bool,
    pub restrict_public_buckets: bool,
}

impl From<&PublicAccessBlockConfig> for StoredPublicAccessBlock {
    fn from(p: &PublicAccessBlockConfig) -> Self {
        Self {
            block_public_acls: p.block_public_acls,
            ignore_public_acls: p.ignore_public_acls,
            block_public_policy: p.block_public_policy,
            restrict_public_buckets: p.restrict_public_buckets,
        }
    }
}

impl From<StoredPublicAccessBlock> for PublicAccessBlockConfig {
    fn from(p: StoredPublicAccessBlock) -> Self {
        Self {
            block_public_acls: p.block_public_acls,
            ignore_public_acls: p.ignore_public_acls,
            block_public_policy: p.block_public_policy,
            restrict_public_buckets: p.restrict_public_buckets,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredBucketConfig {
    pub bucket: String,
    pub location: String,
    pub creation_date: DateTime<Utc>,
    pub versioning_status: Option<String>,
    pub tags: HashMap<String, String>,
    pub acl: Option<String>,
    pub policy: Option<String>,
    pub accelerate_status: Option<String>,
    pub request_payment_payer: String,
    pub cors_configuration: Option<String>,
    pub encryption_configuration: Option<String>,
    pub logging_configuration: Option<String>,
    pub replication_configuration: Option<String>,
    pub inventory_configurations: HashMap<String, String>,
    pub object_lock_configuration: Option<String>,
    pub website_configuration: Option<String>,
    pub public_access_block: Option<StoredPublicAccessBlock>,
    pub ownership_controls: Option<String>,
    pub analytics_configurations: HashMap<String, String>,
    pub intelligent_tiering_configurations: HashMap<String, String>,
    pub metrics_configurations: HashMap<String, String>,
    pub lifecycle_configuration: Option<String>,
    pub notification_configuration: Option<String>,
    pub abac_status: Option<String>,
    pub metadata_table_configuration: Option<String>,
    pub metadata_configuration: Option<String>,
}

impl StoredBucketConfig {
    pub fn from_state(bucket_name: &str, bs: &BucketState) -> Self {
        Self {
            bucket: bucket_name.to_string(),
            location: bs.bucket.region.clone(),
            creation_date: bs.bucket.creation_date,
            versioning_status: bs.versioning_status.clone(),
            tags: bs.tags.clone(),
            acl: bs.acl.clone(),
            policy: bs.policy.clone(),
            accelerate_status: bs.accelerate_status.clone(),
            request_payment_payer: bs.request_payment_payer.clone(),
            cors_configuration: bs.cors_configuration.clone(),
            encryption_configuration: bs.encryption_configuration.clone(),
            logging_configuration: bs.logging_configuration.clone(),
            replication_configuration: bs.replication_configuration.clone(),
            inventory_configurations: bs.inventory_configurations.clone(),
            object_lock_configuration: bs.object_lock_configuration.clone(),
            website_configuration: bs.website_configuration.clone(),
            public_access_block: bs.public_access_block.as_ref().map(Into::into),
            ownership_controls: bs.ownership_controls.clone(),
            analytics_configurations: bs.analytics_configurations.clone(),
            intelligent_tiering_configurations: bs.intelligent_tiering_configurations.clone(),
            metrics_configurations: bs.metrics_configurations.clone(),
            lifecycle_configuration: bs.lifecycle_configuration.clone(),
            notification_configuration: bs.notification_configuration.clone(),
            abac_status: bs.abac_status.clone(),
            metadata_table_configuration: bs.metadata_table_configuration.clone(),
            metadata_configuration: bs.metadata_configuration.clone(),
        }
    }
}

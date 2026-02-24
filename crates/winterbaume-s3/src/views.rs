//! Serde-compatible view types for S3 state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{BlobBackedService, StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::S3Service;
use crate::state::{BucketState, PublicAccessBlockConfig, S3State};
use crate::types::{Bucket, DeleteMarker, Object};

/// Serializable view of the entire S3 state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3StateView {
    /// Buckets keyed by bucket name.
    #[serde(default)]
    pub buckets: HashMap<String, BucketStateView>,
}

/// Serializable view of a single S3 bucket and its contents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BucketStateView {
    /// Bucket name.
    pub name: String,
    /// AWS region.
    pub region: String,
    /// Creation date in RFC 3339 format.
    pub creation_date: Option<String>,
    /// Objects keyed by object key (current live view).
    #[serde(default)]
    pub objects: HashMap<String, ObjectView>,
    /// Full version history per key, newest-first. Only present when versioning has been active.
    #[serde(default)]
    pub object_history: HashMap<String, Vec<ObjectView>>,
    /// Delete markers per key, newest-first.
    #[serde(default)]
    pub delete_markers: HashMap<String, Vec<DeleteMarkerView>>,
    /// Raw XML lifecycle configuration.
    pub lifecycle_configuration: Option<String>,
    /// Raw XML notification configuration.
    pub notification_configuration: Option<String>,
    /// Versioning status.
    #[serde(default)]
    pub versioning_status: Option<String>,
    /// Tags.
    #[serde(default)]
    pub tags: HashMap<String, String>,
    /// Raw ACL XML.
    #[serde(default)]
    pub acl: Option<String>,
    /// Bucket policy JSON.
    #[serde(default)]
    pub policy: Option<String>,
    /// Accelerate configuration status.
    #[serde(default)]
    pub accelerate_status: Option<String>,
    /// Request payment configuration.
    #[serde(default = "default_request_payment_payer")]
    pub request_payment_payer: String,
    /// CORS configuration XML.
    #[serde(default)]
    pub cors_configuration: Option<String>,
    /// Server-side encryption configuration XML.
    #[serde(default)]
    pub encryption_configuration: Option<String>,
    /// Logging configuration XML.
    #[serde(default)]
    pub logging_configuration: Option<String>,
    /// Replication configuration XML.
    #[serde(default)]
    pub replication_configuration: Option<String>,
    /// Object lock configuration XML.
    #[serde(default)]
    pub object_lock_configuration: Option<String>,
    /// Website configuration XML.
    #[serde(default)]
    pub website_configuration: Option<String>,
    /// Public access block configuration.
    #[serde(default)]
    pub public_access_block: Option<PublicAccessBlockConfigView>,
    /// Ownership controls rule.
    #[serde(default)]
    pub ownership_controls: Option<String>,
    /// Analytics configurations keyed by configuration ID.
    ///
    /// Note: `inventory_configurations` and `multipart_uploads` are intentionally
    /// excluded from the view — inventory configs are transient and multipart uploads are
    /// in-flight transient state that need not survive a server restart.
    #[serde(default)]
    pub analytics_configurations: HashMap<String, String>,
    /// Intelligent tiering configurations keyed by configuration ID.
    #[serde(default)]
    pub intelligent_tiering_configurations: HashMap<String, String>,
    /// Metrics configurations keyed by configuration ID.
    #[serde(default)]
    pub metrics_configurations: HashMap<String, String>,
    /// ABAC status XML.
    #[serde(default)]
    pub abac_status: Option<String>,
    /// Metadata table configuration XML.
    #[serde(default)]
    pub metadata_table_configuration: Option<String>,
    /// Metadata configuration XML.
    #[serde(default)]
    pub metadata_configuration: Option<String>,
}

fn default_request_payment_payer() -> String {
    "BucketOwner".to_string()
}

impl Default for BucketStateView {
    fn default() -> Self {
        Self {
            name: String::new(),
            region: String::new(),
            creation_date: None,
            objects: HashMap::new(),
            object_history: HashMap::new(),
            delete_markers: HashMap::new(),
            lifecycle_configuration: None,
            notification_configuration: None,
            versioning_status: None,
            tags: HashMap::new(),
            acl: None,
            policy: None,
            accelerate_status: None,
            request_payment_payer: default_request_payment_payer(),
            cors_configuration: None,
            encryption_configuration: None,
            logging_configuration: None,
            replication_configuration: None,
            object_lock_configuration: None,
            website_configuration: None,
            public_access_block: None,
            ownership_controls: None,
            analytics_configurations: HashMap::new(),
            intelligent_tiering_configurations: HashMap::new(),
            metrics_configurations: HashMap::new(),
            abac_status: None,
            metadata_table_configuration: None,
            metadata_configuration: None,
        }
    }
}

/// Serializable view of a delete marker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMarkerView {
    pub key: String,
    pub version_id: String,
    pub last_modified: Option<String>,
}

/// Serializable view of a public access block configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicAccessBlockConfigView {
    pub block_public_acls: bool,
    pub ignore_public_acls: bool,
    pub block_public_policy: bool,
    pub restrict_public_buckets: bool,
}

/// Serializable view of an S3 object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectView {
    /// Object key.
    pub key: String,
    /// BlobStore key referencing the object body.
    #[serde(default)]
    pub blob_key: String,
    /// Size of the object body in bytes.
    #[serde(default)]
    pub content_length: u64,
    /// Content type (MIME).
    pub content_type: String,
    /// ETag.
    pub etag: String,
    /// Last modified timestamp in RFC 3339 format.
    pub last_modified: Option<String>,
    /// Storage class.
    pub storage_class: String,
    /// User metadata key-value pairs.
    #[serde(default)]
    pub metadata: Vec<(String, String)>,
    /// Object tags.
    #[serde(default)]
    pub tags: HashMap<String, String>,
    /// Object ACL XML.
    #[serde(default)]
    pub acl: Option<String>,
    /// Legal hold status.
    #[serde(default)]
    pub legal_hold_status: Option<String>,
    /// Object lock retention mode.
    #[serde(default)]
    pub retention_mode: Option<String>,
    /// Retain-until date.
    #[serde(default)]
    pub retain_until_date: Option<String>,
    /// S3 version ID (`"null"` or UUID).
    #[serde(default = "default_null_version_id")]
    pub version_id: String,
    /// BlobStore internal version ID for this specific version's blob.
    #[serde(default)]
    pub blob_version_id: String,
}

fn default_null_version_id() -> String {
    "null".to_string()
}

// ---------------------------------------------------------------------------
// StatefulService implementation
// ---------------------------------------------------------------------------

impl S3Service {
    /// Build the state view and collect blob keys from an already-locked state.
    /// Shared by `snapshot()` and `snapshot_with_blobs()`.
    fn snapshot_inner(state: &S3State) -> (S3StateView, Vec<(String, String)>) {
        let mut blob_keys: Vec<(String, String)> = Vec::new();
        let mut buckets = HashMap::new();

        let obj_to_view = |obj: &Object, blob_keys: &mut Vec<(String, String)>| -> ObjectView {
            if !obj.blob_key.is_empty() {
                blob_keys.push((obj.blob_key.clone(), obj.blob_version_id.clone()));
            }
            ObjectView {
                key: obj.key.clone(),
                blob_key: obj.blob_key.clone(),
                content_length: obj.content_length,
                content_type: obj.content_type.clone(),
                etag: obj.etag.clone(),
                last_modified: Some(obj.last_modified.to_rfc3339()),
                storage_class: obj.storage_class.clone(),
                metadata: obj.metadata.clone(),
                tags: obj.tags.clone(),
                acl: obj.acl.clone(),
                legal_hold_status: obj.legal_hold_status.clone(),
                retention_mode: obj.retention_mode.clone(),
                retain_until_date: obj.retain_until_date.clone(),
                version_id: obj.version_id.clone(),
                blob_version_id: obj.blob_version_id.clone(),
            }
        };

        for (name, bs) in &state.buckets {
            let mut objects = HashMap::new();
            for (key, obj) in &bs.objects {
                objects.insert(key.clone(), obj_to_view(obj, &mut blob_keys));
            }

            let mut object_history: HashMap<String, Vec<ObjectView>> = HashMap::new();
            for (key, versions) in &bs.object_history {
                let views: Vec<ObjectView> = versions
                    .iter()
                    .map(|obj| obj_to_view(obj, &mut blob_keys))
                    .collect();
                object_history.insert(key.clone(), views);
            }

            let mut delete_markers_view: HashMap<String, Vec<DeleteMarkerView>> = HashMap::new();
            for (key, markers) in &bs.delete_markers {
                let views: Vec<DeleteMarkerView> = markers
                    .iter()
                    .map(|m| DeleteMarkerView {
                        key: m.key.clone(),
                        version_id: m.version_id.clone(),
                        last_modified: Some(m.last_modified.to_rfc3339()),
                    })
                    .collect();
                delete_markers_view.insert(key.clone(), views);
            }

            buckets.insert(
                name.clone(),
                BucketStateView {
                    name: bs.bucket.name.clone(),
                    region: bs.bucket.region.clone(),
                    creation_date: Some(bs.bucket.creation_date.to_rfc3339()),
                    objects,
                    object_history,
                    delete_markers: delete_markers_view,
                    lifecycle_configuration: bs.lifecycle_configuration.clone(),
                    notification_configuration: bs.notification_configuration.clone(),
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
                    object_lock_configuration: bs.object_lock_configuration.clone(),
                    website_configuration: bs.website_configuration.clone(),
                    public_access_block: bs.public_access_block.as_ref().map(|p| {
                        PublicAccessBlockConfigView {
                            block_public_acls: p.block_public_acls,
                            ignore_public_acls: p.ignore_public_acls,
                            block_public_policy: p.block_public_policy,
                            restrict_public_buckets: p.restrict_public_buckets,
                        }
                    }),
                    ownership_controls: bs.ownership_controls.clone(),
                    analytics_configurations: bs.analytics_configurations.clone(),
                    intelligent_tiering_configurations: bs
                        .intelligent_tiering_configurations
                        .clone(),
                    metrics_configurations: bs.metrics_configurations.clone(),
                    abac_status: bs.abac_status.clone(),
                    metadata_table_configuration: bs.metadata_table_configuration.clone(),
                    metadata_configuration: bs.metadata_configuration.clone(),
                },
            );
        }
        (S3StateView { buckets }, blob_keys)
    }

    fn restore_inner(view: &S3StateView) -> S3State {
        let mut new_state = S3State::default();

        let ov_to_obj = |ov: &ObjectView| -> Object {
            Object {
                key: ov.key.clone(),
                blob_key: ov.blob_key.clone(),
                blob_version_id: ov.blob_version_id.clone(),
                content_length: ov.content_length,
                content_type: ov.content_type.clone(),
                etag: ov.etag.clone(),
                last_modified: ov
                    .last_modified
                    .as_deref()
                    .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(Utc::now),
                storage_class: ov.storage_class.clone(),
                metadata: ov.metadata.clone(),
                tags: ov.tags.clone(),
                acl: ov.acl.clone(),
                legal_hold_status: ov.legal_hold_status.clone(),
                retention_mode: ov.retention_mode.clone(),
                retain_until_date: ov.retain_until_date.clone(),
                version_id: ov.version_id.clone(),
            }
        };

        for (name, bv) in &view.buckets {
            let creation_date = bv
                .creation_date
                .as_deref()
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(Utc::now);

            let mut objects = HashMap::new();
            for (key, ov) in &bv.objects {
                objects.insert(key.clone(), ov_to_obj(ov));
            }

            let mut object_history: HashMap<String, Vec<Object>> = HashMap::new();
            for (key, versions) in &bv.object_history {
                object_history.insert(key.clone(), versions.iter().map(&ov_to_obj).collect());
            }

            let mut delete_markers: HashMap<String, Vec<DeleteMarker>> = HashMap::new();
            for (key, markers) in &bv.delete_markers {
                delete_markers.insert(
                    key.clone(),
                    markers
                        .iter()
                        .map(|m| DeleteMarker {
                            key: m.key.clone(),
                            version_id: m.version_id.clone(),
                            last_modified: m
                                .last_modified
                                .as_deref()
                                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                                .map(|dt| dt.with_timezone(&Utc))
                                .unwrap_or_else(Utc::now),
                        })
                        .collect(),
                );
            }

            new_state.buckets.insert(
                name.clone(),
                BucketState {
                    bucket: Bucket {
                        name: bv.name.clone(),
                        region: bv.region.clone(),
                        creation_date,
                    },
                    objects,
                    object_history,
                    delete_markers,
                    lifecycle_configuration: bv.lifecycle_configuration.clone(),
                    notification_configuration: bv.notification_configuration.clone(),
                    versioning_status: bv.versioning_status.clone(),
                    tags: bv.tags.clone(),
                    acl: bv.acl.clone(),
                    policy: bv.policy.clone(),
                    accelerate_status: bv.accelerate_status.clone(),
                    request_payment_payer: bv.request_payment_payer.clone(),
                    cors_configuration: bv.cors_configuration.clone(),
                    encryption_configuration: bv.encryption_configuration.clone(),
                    logging_configuration: bv.logging_configuration.clone(),
                    replication_configuration: bv.replication_configuration.clone(),
                    // inventory_configurations is intentionally excluded from views —
                    // restored as empty and re-populated by subsequent API calls.
                    inventory_configurations: HashMap::new(),
                    object_lock_configuration: bv.object_lock_configuration.clone(),
                    website_configuration: bv.website_configuration.clone(),
                    public_access_block: bv.public_access_block.as_ref().map(|p| {
                        PublicAccessBlockConfig {
                            block_public_acls: p.block_public_acls,
                            ignore_public_acls: p.ignore_public_acls,
                            block_public_policy: p.block_public_policy,
                            restrict_public_buckets: p.restrict_public_buckets,
                        }
                    }),
                    ownership_controls: bv.ownership_controls.clone(),
                    // multipart_uploads is intentionally excluded from views —
                    // in-flight uploads are transient and need not survive a server restart.
                    multipart_uploads: HashMap::new(),
                    analytics_configurations: bv.analytics_configurations.clone(),
                    intelligent_tiering_configurations: bv
                        .intelligent_tiering_configurations
                        .clone(),
                    metrics_configurations: bv.metrics_configurations.clone(),
                    abac_status: bv.abac_status.clone(),
                    metadata_table_configuration: bv.metadata_table_configuration.clone(),
                    metadata_configuration: bv.metadata_configuration.clone(),
                },
            );
        }

        new_state
    }
}

impl StatefulService for S3Service {
    type StateView = S3StateView;

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
        let new_state = Self::restore_inner(&view);
        {
            let state = self.state.get(account_id, region);
            *state.write().await = new_state;
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
        // Phase 1: put blobs asynchronously (no state lock held)
        struct BucketUpdate {
            name: String,
            bname: String,
            bregion: String,
            creation_date: chrono::DateTime<Utc>,
            objects: Vec<(String, Object)>,
            lifecycle: Option<String>,
            notification: Option<String>,
            versioning_status: Option<String>,
            tags: HashMap<String, String>,
            acl: Option<String>,
            policy: Option<String>,
            accelerate_status: Option<String>,
            request_payment_payer: String,
            cors_configuration: Option<String>,
            encryption_configuration: Option<String>,
            logging_configuration: Option<String>,
            replication_configuration: Option<String>,
            object_lock_configuration: Option<String>,
            website_configuration: Option<String>,
            public_access_block: Option<PublicAccessBlockConfigView>,
            ownership_controls: Option<String>,
            analytics_configurations: HashMap<String, String>,
            intelligent_tiering_configurations: HashMap<String, String>,
            metrics_configurations: HashMap<String, String>,
            abac_status: Option<String>,
            metadata_table_configuration: Option<String>,
            metadata_configuration: Option<String>,
        }
        let mut bucket_updates: Vec<BucketUpdate> = Vec::new();

        for (name, bv) in view.buckets {
            let creation_date = bv
                .creation_date
                .as_deref()
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(Utc::now);

            let ov_to_obj_owned = |ov: ObjectView| -> Object {
                Object {
                    key: ov.key,
                    blob_key: ov.blob_key,
                    blob_version_id: ov.blob_version_id,
                    content_length: ov.content_length,
                    content_type: ov.content_type,
                    etag: ov.etag,
                    last_modified: ov
                        .last_modified
                        .as_deref()
                        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(Utc::now),
                    storage_class: ov.storage_class,
                    metadata: ov.metadata,
                    tags: ov.tags,
                    acl: ov.acl,
                    legal_hold_status: ov.legal_hold_status,
                    retention_mode: ov.retention_mode,
                    retain_until_date: ov.retain_until_date,
                    version_id: ov.version_id,
                }
            };
            let mut objects = Vec::new();
            for (key, ov) in bv.objects {
                objects.push((key, ov_to_obj_owned(ov)));
            }

            bucket_updates.push(BucketUpdate {
                name,
                bname: bv.name,
                bregion: bv.region,
                creation_date,
                objects,
                lifecycle: bv.lifecycle_configuration,
                notification: bv.notification_configuration,
                versioning_status: bv.versioning_status,
                tags: bv.tags,
                acl: bv.acl,
                policy: bv.policy,
                accelerate_status: bv.accelerate_status,
                request_payment_payer: bv.request_payment_payer,
                cors_configuration: bv.cors_configuration,
                encryption_configuration: bv.encryption_configuration,
                logging_configuration: bv.logging_configuration,
                replication_configuration: bv.replication_configuration,
                object_lock_configuration: bv.object_lock_configuration,
                website_configuration: bv.website_configuration,
                public_access_block: bv.public_access_block,
                ownership_controls: bv.ownership_controls,
                analytics_configurations: bv.analytics_configurations,
                intelligent_tiering_configurations: bv.intelligent_tiering_configurations,
                metrics_configurations: bv.metrics_configurations,
                abac_status: bv.abac_status,
                metadata_table_configuration: bv.metadata_table_configuration,
                metadata_configuration: bv.metadata_configuration,
            });
        }

        // Phase 2: update state under write lock (sync)
        {
            let state = self.state.get(account_id, region);
            let mut guard = state.write().await;
            for upd in bucket_updates {
                let bucket_entry = guard
                    .buckets
                    .entry(upd.name)
                    .or_insert_with(|| BucketState {
                        bucket: Bucket {
                            name: upd.bname,
                            region: upd.bregion,
                            creation_date: upd.creation_date,
                        },
                        objects: HashMap::new(),
                        object_history: HashMap::new(),
                        delete_markers: HashMap::new(),
                        lifecycle_configuration: upd.lifecycle,
                        notification_configuration: upd.notification,
                        versioning_status: upd.versioning_status,
                        tags: upd.tags,
                        acl: upd.acl,
                        policy: upd.policy,
                        accelerate_status: upd.accelerate_status,
                        request_payment_payer: upd.request_payment_payer,
                        cors_configuration: upd.cors_configuration,
                        encryption_configuration: upd.encryption_configuration,
                        logging_configuration: upd.logging_configuration,
                        replication_configuration: upd.replication_configuration,
                        inventory_configurations: HashMap::new(),
                        object_lock_configuration: upd.object_lock_configuration,
                        website_configuration: upd.website_configuration,
                        public_access_block: upd.public_access_block.map(|p| {
                            PublicAccessBlockConfig {
                                block_public_acls: p.block_public_acls,
                                ignore_public_acls: p.ignore_public_acls,
                                block_public_policy: p.block_public_policy,
                                restrict_public_buckets: p.restrict_public_buckets,
                            }
                        }),
                        ownership_controls: upd.ownership_controls,
                        // multipart_uploads is intentionally excluded from views —
                        // in-flight uploads are transient and need not survive a server restart.
                        multipart_uploads: HashMap::new(),
                        analytics_configurations: upd.analytics_configurations,
                        intelligent_tiering_configurations: upd.intelligent_tiering_configurations,
                        metrics_configurations: upd.metrics_configurations,
                        abac_status: upd.abac_status,
                        metadata_table_configuration: upd.metadata_table_configuration,
                        metadata_configuration: upd.metadata_configuration,
                    });
                for (key, obj) in upd.objects {
                    bucket_entry.objects.insert(key, obj);
                }
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

impl BlobBackedService for S3Service {
    async fn snapshot_with_blobs(
        &self,
        account_id: &str,
        region: &str,
        visitor: &mut dyn winterbaume_core::BlobVisitor,
    ) -> Result<S3StateView, winterbaume_core::StateViewError> {
        let blobs = self.blobs.get(account_id, region);
        let state = self.state.get(account_id, region);
        let lock = state.read().await;
        let (view, raw_keys) = Self::snapshot_inner(&lock);

        // Deduplicate (blob_key, blob_version_id) pairs; the export key encodes both
        // separated by a NUL byte so restore can split them back out.
        // Legacy objects with an empty blob_version_id are exported under the plain blob_key.
        let mut seen = std::collections::HashSet::new();
        let mut unique_pairs: Vec<(String, String)> = Vec::new();
        for (bkey, bvid) in raw_keys {
            let dedup_key = if bvid.is_empty() {
                bkey.clone()
            } else {
                format!("{}\x00{}", bkey, bvid)
            };
            if seen.insert(dedup_key) {
                unique_pairs.push((bkey, bvid));
            }
        }
        drop(lock); // release read lock before async I/O

        for chunk in unique_pairs.chunks(winterbaume_core::DEFAULT_BLOB_BATCH_SIZE) {
            let mut entries = Vec::with_capacity(chunk.len());
            for (blob_key, blob_version_id) in chunk {
                let (export_key, reader, size) = if blob_version_id.is_empty() {
                    // Legacy path: blob was stored unversioned.
                    let size = blobs
                        .stat(blob_key)
                        .await
                        .map_err(winterbaume_core::StateViewError::Blob)?
                        .map(|s| s.size);
                    let reader = blobs
                        .get_reader(blob_key)
                        .await
                        .map_err(winterbaume_core::StateViewError::Blob)?;
                    (blob_key.clone(), reader, size)
                } else {
                    // Versioned path.
                    let reader = blobs
                        .get_version_reader(blob_key, blob_version_id)
                        .await
                        .map_err(winterbaume_core::StateViewError::Blob)?;
                    let export_key = format!("{}\x00{}", blob_key, blob_version_id);
                    (export_key, reader, None)
                };
                entries.push(winterbaume_core::BlobExportEntry {
                    key: export_key,
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
        // When no snapshot is available (empty view), reconstruct state entirely
        // from the BlobStore sidecars written during previous mutations.
        if view.buckets.is_empty() {
            let recovered = Self::recover_state_from_blobs(&blobs)
                .await
                .map_err(winterbaume_core::StateViewError::Blob)?;
            let state = self.state.get(account_id, region);
            let mut state = state.write().await;
            *state = recovered;
            return Ok(());
        }

        let new_state = Self::restore_inner(&view);

        // Collect all (blob_key, blob_version_id, is_current) triples from the new state.
        // is_current is true when this version is the live object (referenced by objects map).
        let mut to_import: Vec<(String, String, bool)> = Vec::new();
        let mut seen = std::collections::HashSet::new();
        for bs in new_state.buckets.values() {
            // Current live objects.
            for obj in bs.objects.values() {
                if obj.blob_key.is_empty() {
                    continue;
                }
                let dedup = if obj.blob_version_id.is_empty() {
                    obj.blob_key.clone()
                } else {
                    format!("{}\x00{}", obj.blob_key, obj.blob_version_id)
                };
                if seen.insert(dedup) {
                    to_import.push((obj.blob_key.clone(), obj.blob_version_id.clone(), true));
                }
            }
            // Historical versions.
            for versions in bs.object_history.values() {
                for obj in versions {
                    if obj.blob_key.is_empty() {
                        continue;
                    }
                    let dedup = if obj.blob_version_id.is_empty() {
                        obj.blob_key.clone()
                    } else {
                        format!("{}\x00{}", obj.blob_key, obj.blob_version_id)
                    };
                    if seen.insert(dedup) {
                        // Only mark as current if not already claimed by an objects entry.
                        to_import.push((obj.blob_key.clone(), obj.blob_version_id.clone(), false));
                    }
                }
            }
        }

        // Import each blob version at the exact version_id so state references remain valid.
        for (blob_key, blob_version_id, is_current) in &to_import {
            if blob_version_id.is_empty() {
                // Legacy path: plain blob_key with no version tracking.
                let mut reader = source.fetch(blob_key.clone()).await?;
                blobs
                    .put_from(blob_key, &mut reader)
                    .await
                    .map_err(winterbaume_core::StateViewError::Blob)?;
            } else {
                let export_key = format!("{}\x00{}", blob_key, blob_version_id);
                let mut reader = source.fetch(export_key).await?;
                blobs
                    .import_version(blob_key, blob_version_id, &mut reader, *is_current)
                    .await
                    .map_err(winterbaume_core::StateViewError::Blob)?;
            }
        }

        let state = self.state.get(account_id, region);
        let mut state = state.write().await;
        *state = new_state;
        Ok(())
    }
}

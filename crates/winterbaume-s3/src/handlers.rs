use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use bytes::Bytes;
use tokio::io::AsyncReadExt;
use winterbaume_core::{
    BackendState, BlobStore, BlobStoreMap, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse,
    MockService, StateChangeNotifier, StatefulService, Vfs, VfsError,
};

use crate::state::{S3Error, S3State, compute_etag, compute_etag_raw, no_such_bucket};
use crate::views::S3StateView;
use crate::wire;

/// Compute the deterministic BlobStore key for an S3 object.
///
/// All `/` characters in the S3 key are percent-encoded to `%2F`, so the
/// first `/` in the resulting string always separates the bucket name from
/// the encoded key.  This means any two distinct S3 keys always map to
/// distinct blob keys, with no prefix-collision risk in the VFS layer.
///
/// Example: bucket `"b"`, key `"foo/bar"` → `"b/foo%2Fbar"`.
fn encode_blob_key(bucket_name: &str, s3_key: &str) -> String {
    format!("{}/{}", bucket_name, s3_key.replace('/', "%2F"))
}

/// S3 service handler that processes REST-XML protocol requests.
pub struct S3Service {
    pub(crate) state: Arc<BackendState<S3State>>,
    pub(crate) notifier: StateChangeNotifier<S3StateView>,
    pub(crate) blobs: Arc<BlobStoreMap>,
}

impl S3Service {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
            blobs: Arc::new(BlobStoreMap::mem("s3")),
        }
    }

    /// Create an S3 service backed by the given VFS for blob storage.
    pub fn with_vfs(vfs: Arc<dyn Vfs>) -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
            blobs: Arc::new(BlobStoreMap::new(vfs, "s3")),
        }
    }

    /// Recover state from the blob store.
    ///
    /// Discovers all `(account_id, region)` pairs stored in the VFS and
    /// recovers each one.  If no scoped children are found, falls back to
    /// the legacy flat namespace for backward compatibility.
    pub async fn recover(&self) -> Result<(), VfsError> {
        let children = self.blobs.list_children().await?;
        for (account_id, region) in &children {
            let scoped = self.blobs.get(account_id, region);
            let recovered = Self::recover_state_from_blobs(&scoped).await?;
            let state = self.state.get(account_id, region);
            *state.write().await = recovered;
        }
        Ok(())
    }

    /// Create an S3 service backed by the given VFS for blob storage, and
    /// restores the state from the metadata there.
    pub async fn recover_with_vfs(vfs: Arc<dyn Vfs>) -> Result<Self, VfsError> {
        let blobs = BlobStoreMap::new(vfs, "s3");
        let children = blobs.list_children().await?;
        let mut entries: Vec<((String, String), S3State)> = Vec::with_capacity(children.len());
        for (account_id, region) in &children {
            let scoped = blobs.get(account_id, region);
            let recovered = Self::recover_state_from_blobs(&scoped).await?;
            entries.push(((account_id.clone(), region.clone()), recovered));
        }
        Ok(Self {
            state: Arc::new(BackendState::from_iter(entries)),
            notifier: StateChangeNotifier::new(),
            blobs: Arc::new(blobs),
        })
    }

    /// Returns sorted `(account_id, region)` pairs that have state.
    pub fn scopes_with_state(&self) -> Vec<(String, String)> {
        self.state.scopes_with_state()
    }
}

impl Default for S3Service {
    fn default() -> Self {
        Self::new()
    }
}

impl S3Service {
    /// Write (or overwrite) the bucket-config stored record after any mutation that
    /// changes bucket-level configuration.  Errors are silently ignored since
    /// the stored record is a recovery aid, not a primary data path.
    async fn persist_bucket_config(
        &self,
        blobs: &BlobStore,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) {
        let value = {
            let guard = state.read().await;
            guard.buckets.get(bucket_name).map(|bs| {
                let meta = crate::stored::StoredBucketConfig::from_state(bucket_name, bs);
                serde_json::to_value(&meta).ok()
            })
        };
        if let Some(Some(v)) = value {
            let _ = blobs.put_bucket_config(bucket_name, &v).await;
        }
    }

    /// Reconstruct [`S3State`] solely from the BlobStore stored records.
    ///
    /// This is the fallback recovery path used when no JSON snapshot is
    /// available.  It walks the three stored-record namespaces (`bucket-config`,
    /// `version-meta`, `delete-markers`) and reassembles the full in-memory
    /// state.
    ///
    /// Errors reading individual stored records are silently skipped so that a
    /// partially-corrupted store still recovers as much as possible.
    pub(crate) async fn recover_state_from_blobs(blobs: &BlobStore) -> Result<S3State, VfsError> {
        use std::collections::HashMap;

        use crate::state::{BucketState, PublicAccessBlockConfig};
        use crate::stored::{StoredBucketConfig, StoredDeleteMarker, StoredObjectVersion};
        use crate::types::{Bucket, DeleteMarker, Object};

        let mut state = S3State::default();

        // --- Step 1: load bucket configurations ---
        let bucket_names = blobs.list_bucket_configs().await?;
        for bucket_name in &bucket_names {
            let Ok(json) = blobs.get_bucket_config(bucket_name).await else {
                continue;
            };
            let Ok(meta) = serde_json::from_value::<StoredBucketConfig>(json) else {
                continue;
            };
            let bucket = Bucket {
                name: bucket_name.clone(),
                region: meta.location.clone(),
                creation_date: meta.creation_date,
            };
            let bs = BucketState {
                bucket,
                objects: HashMap::new(),
                object_history: HashMap::new(),
                delete_markers: HashMap::new(),
                lifecycle_configuration: meta.lifecycle_configuration,
                notification_configuration: meta.notification_configuration,
                versioning_status: meta.versioning_status,
                tags: meta.tags,
                acl: meta.acl,
                policy: meta.policy,
                accelerate_status: meta.accelerate_status,
                request_payment_payer: meta.request_payment_payer,
                cors_configuration: meta.cors_configuration,
                encryption_configuration: meta.encryption_configuration,
                logging_configuration: meta.logging_configuration,
                replication_configuration: meta.replication_configuration,
                inventory_configurations: meta.inventory_configurations,
                object_lock_configuration: meta.object_lock_configuration,
                website_configuration: meta.website_configuration,
                public_access_block: meta.public_access_block.map(|p| PublicAccessBlockConfig {
                    block_public_acls: p.block_public_acls,
                    ignore_public_acls: p.ignore_public_acls,
                    block_public_policy: p.block_public_policy,
                    restrict_public_buckets: p.restrict_public_buckets,
                }),
                ownership_controls: meta.ownership_controls,
                multipart_uploads: HashMap::new(),
                analytics_configurations: meta.analytics_configurations,
                intelligent_tiering_configurations: meta.intelligent_tiering_configurations,
                metrics_configurations: meta.metrics_configurations,
                abac_status: meta.abac_status,
                metadata_table_configuration: meta.metadata_table_configuration,
                metadata_configuration: meta.metadata_configuration,
            };
            state.buckets.insert(bucket_name.clone(), bs);
        }

        // --- Step 2: load object version metadata ---
        let version_pairs = blobs.list_version_metas("").await?;
        for (blob_key, blob_version_id) in version_pairs {
            let Ok(json) = blobs.get_version_meta(&blob_key, &blob_version_id).await else {
                continue;
            };
            let Ok(meta) = serde_json::from_value::<StoredObjectVersion>(json) else {
                continue;
            };
            let bucket_name = &meta.bucket;
            let Some(bs) = state.buckets.get_mut(bucket_name) else {
                continue;
            };
            let obj = Object {
                key: meta.key.clone(),
                blob_key: meta.blob_key.clone(),
                version_id: meta.version_id.clone(),
                blob_version_id: meta.blob_version_id.clone(),
                etag: meta.etag.clone(),
                content_type: meta.content_type.clone(),
                content_length: meta.content_length,
                last_modified: meta.last_modified,
                storage_class: meta.storage_class.clone(),
                metadata: meta.metadata.clone(),
                tags: meta.tags.clone(),
                acl: meta.acl.clone(),
                legal_hold_status: meta.legal_hold_status.clone(),
                retention_mode: meta.retention_mode.clone(),
                retain_until_date: meta.retain_until_date.clone(),
            };
            bs.object_history
                .entry(meta.key.clone())
                .or_default()
                .push(obj);
        }

        // --- Step 3: sort history newest-first; populate live objects map ---
        for bs in state.buckets.values_mut() {
            for (key, versions) in &mut bs.object_history {
                versions.sort_by_key(|b| std::cmp::Reverse(b.last_modified));
                // The live "current" object is the newest version.
                bs.objects.insert(key.clone(), versions[0].clone());
            }
        }

        // --- Step 4: load delete markers ---
        let dm_pairs = blobs.list_delete_marker_metas("").await?;
        for (blob_key, dm_version_id) in dm_pairs {
            let Ok(json) = blobs
                .get_delete_marker_meta(&blob_key, &dm_version_id)
                .await
            else {
                continue;
            };
            let Ok(meta) = serde_json::from_value::<StoredDeleteMarker>(json) else {
                continue;
            };
            let bucket_name = &meta.bucket;
            let Some(bs) = state.buckets.get_mut(bucket_name) else {
                continue;
            };
            let dm = DeleteMarker {
                key: meta.key.clone(),
                version_id: meta.version_id.clone(),
                last_modified: meta.last_modified,
            };
            bs.delete_markers
                .entry(meta.key.clone())
                .or_default()
                .push(dm);
        }

        // --- Step 5: sort delete markers newest-first; remove live objects
        //             that are hidden behind a newer delete marker ---
        for bs in state.buckets.values_mut() {
            for (key, markers) in &mut bs.delete_markers {
                markers.sort_by_key(|b| std::cmp::Reverse(b.last_modified));
                // If the newest delete marker is newer than the newest version,
                // the key is logically deleted in the live view.
                let newest_dm_time = markers[0].last_modified;
                let newest_version_time = bs
                    .object_history
                    .get(key)
                    .and_then(|v| v.first())
                    .map(|o| o.last_modified);
                if newest_version_time.is_none_or(|vt| newest_dm_time >= vt) {
                    bs.objects.remove(key);
                }
            }
        }

        Ok(state)
    }
}

impl MockService for S3Service {
    fn service_name(&self) -> &str {
        "s3"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            // Virtual-hosted style: bucket.s3.region.amazonaws.com
            r"https?://[a-zA-Z0-9\-_.]+\.s3[.\-][a-zA-Z0-9\-]+\.amazonaws\.com",
            // Virtual-hosted style global: bucket.s3.amazonaws.com
            r"https?://[a-zA-Z0-9\-_.]+\.s3\.amazonaws\.com",
            // Path-style: s3.region.amazonaws.com/bucket
            r"https?://s3[.\-][a-zA-Z0-9\-]+\.amazonaws\.com",
            // Path-style global: s3.amazonaws.com/bucket
            r"https?://s3\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

/// Parsed S3 request target.
struct S3Target {
    bucket: Option<String>,
    key: Option<String>,
    region: String,
}

impl S3Service {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        // FIX(terraform-e2e): s3control (account-level S3 Control API) requests arrive here
        //   because s3control signs with service name "s3" in the credential scope, so the
        //   router dispatches them to S3Service. S3 Control paths use the /v20180820/ prefix.
        //   Without this guard, parse_s3_target would misinterpret "v20180820" as a bucket name.
        if let Some((_, path)) = extract_host_and_path(&request.uri) {
            let path_no_qs = path.split('?').next().unwrap_or(path);
            if path_no_qs.starts_with("/v20180820/") {
                return self.dispatch_s3control(request).await;
            }
        }

        let mut target = match parse_s3_target(&request.uri) {
            Some(t) => t,
            None => {
                return s3_error_response(&S3Error::InvalidRequest {
                    resource: request.uri.clone(),
                });
            }
        };

        // If region is the default, try extracting from Authorization header
        if target.region == "us-east-1" {
            if let Some(region) =
                winterbaume_core::auth::extract_region_from_headers(&request.headers)
            {
                target.region = region;
            }
        }

        let region = &target.region;
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, region);
        let blobs = self.blobs.get(account_id, region);

        let method = request.method.as_str();

        // Parse query string for list-type, etc.
        let query = parse_query_from_uri(&request.uri);

        let response = match (&target.bucket, &target.key, method) {
            // Service-level operations (no bucket)
            (None, _, "GET") => {
                if query.get("x-id").map(|s| s.as_str()) == Some("ListDirectoryBuckets") {
                    self.handle_list_directory_buckets().await
                } else {
                    self.handle_list_buckets(&state).await
                }
            }
            // WriteGetObjectResponse (POST /WriteGetObjectResponse)
            (None, Some(key), "POST") if key == "WriteGetObjectResponse" => {
                wire::serialize_write_get_object_response_response()
            }

            // Bucket operations (no key)
            (Some(bucket), None, "PUT") => {
                if query.contains_key("lifecycle") {
                    // PUT /{Bucket}?lifecycle - PutBucketLifecycleConfiguration
                    self.handle_put_bucket_lifecycle_configuration(&state, bucket, request.body)
                        .await
                } else if query.contains_key("notification") {
                    // PUT /{Bucket}?notification - PutBucketNotificationConfiguration
                    self.handle_put_bucket_notification_configuration(&state, bucket, request.body)
                        .await
                } else if query.contains_key("inventory") {
                    // PUT /{Bucket}?inventory&id=Id - PutBucketInventoryConfiguration
                    self.handle_put_bucket_inventory_configuration(&state, bucket, &query, &request)
                        .await
                } else if query.contains_key("replication") {
                    // PUT /{Bucket}?replication - PutBucketReplication
                    self.handle_put_bucket_replication(&state, bucket, request.body)
                        .await
                } else if query.contains_key("tagging") {
                    // PUT /{Bucket}?tagging - PutBucketTagging
                    self.handle_put_bucket_tagging(&state, bucket, &query, &request)
                        .await
                } else if query.contains_key("versioning") {
                    // PUT /{Bucket}?versioning - PutBucketVersioning
                    self.handle_put_bucket_versioning(&state, bucket, request.body)
                        .await
                } else if query.contains_key("acl") {
                    // PUT /{Bucket}?acl - PutBucketAcl
                    self.handle_put_bucket_acl(&state, bucket, request.body)
                        .await
                } else if query.contains_key("accelerate") {
                    // PUT /{Bucket}?accelerate - PutBucketAccelerateConfiguration
                    self.handle_put_bucket_accelerate_configuration(&state, bucket, request.body)
                        .await
                } else if query.contains_key("cors") {
                    // PUT /{Bucket}?cors - PutBucketCors
                    self.handle_put_bucket_cors(&state, bucket, request.body)
                        .await
                } else if query.contains_key("encryption") {
                    // PUT /{Bucket}?encryption - PutBucketEncryption
                    self.handle_put_bucket_encryption(&state, bucket, request.body)
                        .await
                } else if query.contains_key("logging") {
                    // PUT /{Bucket}?logging - PutBucketLogging
                    self.handle_put_bucket_logging(&state, bucket, request.body)
                        .await
                } else if query.contains_key("policy") {
                    // PUT /{Bucket}?policy - PutBucketPolicy
                    self.handle_put_bucket_policy(&state, bucket, request.body)
                        .await
                } else if query.contains_key("requestPayment") {
                    // PUT /{Bucket}?requestPayment - PutBucketRequestPayment
                    self.handle_put_bucket_request_payment(&state, bucket, request.body)
                        .await
                } else if query.contains_key("website") {
                    // PUT /{Bucket}?website - PutBucketWebsite
                    self.handle_put_bucket_website(&state, bucket, request.body)
                        .await
                } else if query.contains_key("object-lock") {
                    // PUT /{Bucket}?object-lock - PutObjectLockConfiguration
                    self.handle_put_object_lock_configuration(&state, bucket, request.body)
                        .await
                } else if query.contains_key("publicAccessBlock") {
                    // PUT /{Bucket}?publicAccessBlock - PutPublicAccessBlock
                    self.handle_put_public_access_block(&state, bucket, request.body)
                        .await
                } else if query.contains_key("ownershipControls") {
                    // PUT /{Bucket}?ownershipControls - PutBucketOwnershipControls
                    self.handle_put_bucket_ownership_controls(&state, bucket, request.body)
                        .await
                } else if query.contains_key("analytics") {
                    // PUT /{Bucket}?analytics - PutBucketAnalyticsConfiguration
                    self.handle_put_bucket_analytics_configuration(&state, bucket, &query, &request)
                        .await
                } else if query.contains_key("intelligent-tiering") {
                    // PUT /{Bucket}?intelligent-tiering - PutBucketIntelligentTieringConfiguration
                    self.handle_put_bucket_intelligent_tiering_configuration(
                        &state, bucket, &query, &request,
                    )
                    .await
                } else if query.contains_key("metrics") {
                    // PUT /{Bucket}?metrics - PutBucketMetricsConfiguration
                    self.handle_put_bucket_metrics_configuration(&state, bucket, &query, &request)
                        .await
                } else if query.contains_key("abac") {
                    // PUT /{Bucket}?abac - PutBucketAbac
                    self.handle_put_bucket_abac(&state, bucket, &request).await
                } else if query.contains_key("metadataInventoryTable") {
                    // PUT /{Bucket}?metadataInventoryTable - UpdateBucketMetadataInventoryTableConfiguration
                    self.handle_update_bucket_metadata_inventory_table_configuration(
                        &state, bucket, &request,
                    )
                    .await
                } else if query.contains_key("metadataJournalTable") {
                    // PUT /{Bucket}?metadataJournalTable - UpdateBucketMetadataJournalTableConfiguration
                    self.handle_update_bucket_metadata_journal_table_configuration(
                        &state, bucket, &request,
                    )
                    .await
                } else {
                    // PUT /{Bucket} - CreateBucket
                    self.handle_create_bucket(&blobs, &state, bucket, region)
                        .await
                }
            }
            (Some(bucket), None, "POST") => {
                if query.contains_key("delete") {
                    // POST /{Bucket}?delete - DeleteObjects
                    self.handle_delete_objects(&blobs, &state, bucket, &query, &request)
                        .await
                } else if query.contains_key("metadataTable") {
                    // POST /{Bucket}?metadataTable - CreateBucketMetadataTableConfiguration
                    self.handle_create_bucket_metadata_table_configuration(&state, bucket, &request)
                        .await
                } else if query.contains_key("metadataConfiguration") {
                    // POST /{Bucket}?metadataConfiguration - CreateBucketMetadataConfiguration
                    self.handle_create_bucket_metadata_configuration(&state, bucket, &request)
                        .await
                } else {
                    s3_error_response(&S3Error::MethodNotAllowed {
                        method: method.to_string(),
                        resource: request.uri.clone(),
                    })
                }
            }
            (Some(bucket), None, "HEAD") => self.handle_head_bucket(&state, bucket).await,
            (Some(bucket), None, "DELETE") => {
                if query.contains_key("tagging") {
                    // DELETE /{Bucket}?tagging - DeleteBucketTagging
                    self.handle_delete_bucket_tagging(&state, bucket).await
                } else if query.contains_key("cors") {
                    // DELETE /{Bucket}?cors - DeleteBucketCors
                    self.handle_delete_bucket_cors(&state, bucket).await
                } else if query.contains_key("policy") {
                    // DELETE /{Bucket}?policy - DeleteBucketPolicy
                    self.handle_delete_bucket_policy(&state, bucket).await
                } else if query.contains_key("encryption") {
                    // DELETE /{Bucket}?encryption - DeleteBucketEncryption
                    self.handle_delete_bucket_encryption(&state, bucket).await
                } else if query.contains_key("lifecycle") {
                    // DELETE /{Bucket}?lifecycle - DeleteBucketLifecycle
                    self.handle_delete_bucket_lifecycle(&state, bucket).await
                } else if query.contains_key("replication") {
                    // DELETE /{Bucket}?replication - DeleteBucketReplication
                    self.handle_delete_bucket_replication(&state, bucket).await
                } else if query.contains_key("website") {
                    // DELETE /{Bucket}?website - DeleteBucketWebsite
                    self.handle_delete_bucket_website(&state, bucket).await
                } else if query.contains_key("publicAccessBlock") {
                    // DELETE /{Bucket}?publicAccessBlock - DeletePublicAccessBlock
                    self.handle_delete_public_access_block(&state, bucket).await
                } else if query.contains_key("ownershipControls") {
                    // DELETE /{Bucket}?ownershipControls - DeleteBucketOwnershipControls
                    self.handle_delete_bucket_ownership_controls(&state, bucket)
                        .await
                } else if query.contains_key("inventory") {
                    // DELETE /{Bucket}?inventory - DeleteBucketInventoryConfiguration
                    self.handle_delete_bucket_inventory_configuration(
                        &state, bucket, &query, &request,
                    )
                    .await
                } else if query.contains_key("analytics") {
                    // DELETE /{Bucket}?analytics - DeleteBucketAnalyticsConfiguration
                    self.handle_delete_bucket_analytics_configuration(
                        &state, bucket, &query, &request,
                    )
                    .await
                } else if query.contains_key("intelligent-tiering") {
                    // DELETE /{Bucket}?intelligent-tiering - DeleteBucketIntelligentTieringConfiguration
                    self.handle_delete_bucket_intelligent_tiering_configuration(
                        &state, bucket, &query, &request,
                    )
                    .await
                } else if query.contains_key("metrics") {
                    // DELETE /{Bucket}?metrics - DeleteBucketMetricsConfiguration
                    self.handle_delete_bucket_metrics_configuration(
                        &state, bucket, &query, &request,
                    )
                    .await
                } else if query.contains_key("metadataConfiguration") {
                    // DELETE /{Bucket}?metadataConfiguration - DeleteBucketMetadataConfiguration
                    self.handle_delete_bucket_metadata_configuration(&state, bucket)
                        .await
                } else if query.contains_key("metadataTable") {
                    // DELETE /{Bucket}?metadataTable - DeleteBucketMetadataTableConfiguration
                    self.handle_delete_bucket_metadata_table_configuration(&state, bucket)
                        .await
                } else {
                    // DELETE /{Bucket} - DeleteBucket
                    self.handle_delete_bucket(&blobs, &state, bucket).await
                }
            }
            (Some(bucket), None, "GET") => {
                if query.contains_key("lifecycle") {
                    // GET /{Bucket}?lifecycle - GetBucketLifecycleConfiguration
                    self.handle_get_bucket_lifecycle_configuration(&state, bucket)
                        .await
                } else if query.contains_key("notification") {
                    // GET /{Bucket}?notification - GetBucketNotificationConfiguration
                    self.handle_get_bucket_notification_configuration(&state, bucket)
                        .await
                } else if query.contains_key("inventory") {
                    if query.contains_key("id") {
                        // GET /{Bucket}?inventory&id=Id - GetBucketInventoryConfiguration
                        self.handle_get_bucket_inventory_configuration(&state, bucket, &query)
                            .await
                    } else {
                        // GET /{Bucket}?inventory - ListBucketInventoryConfigurations
                        self.handle_list_bucket_inventory_configurations(&state, bucket)
                            .await
                    }
                } else if query.contains_key("location") {
                    // GET /{Bucket}?location - GetBucketLocation
                    self.handle_get_bucket_location(&state, bucket).await
                } else if query.contains_key("versioning") {
                    // GET /{Bucket}?versioning - GetBucketVersioning
                    self.handle_get_bucket_versioning(&state, bucket).await
                } else if query.contains_key("acl") {
                    // GET /{Bucket}?acl - GetBucketAcl
                    self.handle_get_bucket_acl(&state, bucket).await
                } else if query.contains_key("policy") {
                    // GET /{Bucket}?policy - GetBucketPolicy
                    self.handle_get_bucket_policy(&state, bucket).await
                } else if query.contains_key("tagging") {
                    // GET /{Bucket}?tagging - GetBucketTagging
                    self.handle_get_bucket_tagging(&state, bucket).await
                } else if query.contains_key("accelerate") {
                    // GET /{Bucket}?accelerate - GetBucketAccelerateConfiguration
                    self.handle_get_bucket_accelerate_configuration(&state, bucket)
                        .await
                } else if query.contains_key("requestPayment") {
                    // GET /{Bucket}?requestPayment - GetBucketRequestPayment
                    self.handle_get_bucket_request_payment(&state, bucket).await
                } else if query.contains_key("cors") {
                    // GET /{Bucket}?cors - GetBucketCors
                    self.handle_get_bucket_cors(&state, bucket).await
                } else if query.contains_key("encryption") {
                    // GET /{Bucket}?encryption - GetBucketEncryption
                    self.handle_get_bucket_encryption(&state, bucket).await
                } else if query.contains_key("object-lock") {
                    // GET /{Bucket}?object-lock - GetObjectLockConfiguration
                    self.handle_get_bucket_object_lock_configuration(&state, bucket)
                        .await
                } else if query.contains_key("logging") {
                    // GET /{Bucket}?logging - GetBucketLogging
                    self.handle_get_bucket_logging(&state, bucket).await
                } else if query.contains_key("replication") {
                    // GET /{Bucket}?replication - GetBucketReplication
                    self.handle_get_bucket_replication(&state, bucket).await
                } else if query.contains_key("website") {
                    // GET /{Bucket}?website - GetBucketWebsite
                    self.handle_get_bucket_website(&state, bucket).await
                } else if query.contains_key("policyStatus") {
                    // GET /{Bucket}?policyStatus - GetBucketPolicyStatus
                    self.handle_get_bucket_policy_status(&state, bucket).await
                } else if query.contains_key("ownershipControls") {
                    // GET /{Bucket}?ownershipControls - GetBucketOwnershipControls
                    self.handle_get_bucket_ownership_controls(&state, bucket)
                        .await
                } else if query.contains_key("publicAccessBlock") {
                    // GET /{Bucket}?publicAccessBlock - GetPublicAccessBlock
                    self.handle_get_public_access_block(&state, bucket).await
                } else if query.contains_key("versions") {
                    // GET /{Bucket}?versions - ListObjectVersions
                    self.handle_list_object_versions(&state, bucket, &query, &request)
                        .await
                } else if query.contains_key("analytics") {
                    if query.contains_key("id") {
                        // GET /{Bucket}?analytics&id=Id - GetBucketAnalyticsConfiguration
                        self.handle_get_bucket_analytics_configuration(&state, bucket, &query)
                            .await
                    } else {
                        // GET /{Bucket}?analytics - ListBucketAnalyticsConfigurations
                        self.handle_list_bucket_analytics_configurations(&state, bucket)
                            .await
                    }
                } else if query.contains_key("intelligent-tiering") {
                    if query.contains_key("id") {
                        // GET /{Bucket}?intelligent-tiering&id=Id - GetBucketIntelligentTieringConfiguration
                        self.handle_get_bucket_intelligent_tiering_configuration(
                            &state, bucket, &query,
                        )
                        .await
                    } else {
                        // GET /{Bucket}?intelligent-tiering - ListBucketIntelligentTieringConfigurations
                        self.handle_list_bucket_intelligent_tiering_configurations(&state, bucket)
                            .await
                    }
                } else if query.contains_key("metrics") {
                    if query.contains_key("id") {
                        // GET /{Bucket}?metrics&id=Id - GetBucketMetricsConfiguration
                        self.handle_get_bucket_metrics_configuration(&state, bucket, &query)
                            .await
                    } else {
                        // GET /{Bucket}?metrics - ListBucketMetricsConfigurations
                        self.handle_list_bucket_metrics_configurations(&state, bucket)
                            .await
                    }
                } else if query.contains_key("abac") {
                    // GET /{Bucket}?abac - GetBucketAbac
                    self.handle_get_bucket_abac(&state, bucket).await
                } else if query.contains_key("metadataConfiguration") {
                    // GET /{Bucket}?metadataConfiguration - GetBucketMetadataConfiguration
                    self.handle_get_bucket_metadata_configuration(&state, bucket)
                        .await
                } else if query.contains_key("metadataTable") {
                    // GET /{Bucket}?metadataTable - GetBucketMetadataTableConfiguration
                    self.handle_get_bucket_metadata_table_configuration(&state, bucket)
                        .await
                } else if query.contains_key("session") {
                    // GET /{Bucket}?session - CreateSession
                    self.handle_create_session(&state, bucket).await
                } else if query.contains_key("uploads") {
                    // GET /{Bucket}?uploads - ListMultipartUploads
                    self.handle_list_multipart_uploads(&state, bucket, &query, &request)
                        .await
                } else if query.get("list-type").is_some_and(|value| value == "2") {
                    // GET /{Bucket}?list-type=2 - ListObjectsV2
                    self.handle_list_objects_v2(&state, bucket, &query).await
                } else {
                    // GET /{Bucket} - ListObjects
                    self.handle_list_objects(&state, bucket, &query).await
                }
            }

            // Object operations (with key)
            (Some(bucket), Some(key), "PUT") => {
                if query.contains_key("partNumber") && query.contains_key("uploadId") {
                    if request.headers.contains_key("x-amz-copy-source") {
                        // PUT /{Bucket}/{Key}?partNumber=N&uploadId=Id with x-amz-copy-source - UploadPartCopy
                        self.handle_upload_part_copy(&blobs, &state, bucket, key, &query, &request)
                            .await
                    } else {
                        // PUT /{Bucket}/{Key}?partNumber=N&uploadId=Id - UploadPart
                        self.handle_upload_part(&blobs, &state, bucket, key, &query, &request)
                            .await
                    }
                } else if query.contains_key("acl") {
                    // PUT /{Bucket}/{Key}?acl - PutObjectAcl
                    self.handle_put_object_acl(&state, bucket, key, &request)
                        .await
                } else if query.contains_key("tagging") {
                    // PUT /{Bucket}/{Key}?tagging - PutObjectTagging
                    self.handle_put_object_tagging(&state, bucket, key, &query, &request)
                        .await
                } else if query.contains_key("legal-hold") {
                    // PUT /{Bucket}/{Key}?legal-hold - PutObjectLegalHold
                    self.handle_put_object_legal_hold(&state, bucket, key, &query, &request)
                        .await
                } else if query.contains_key("retention") {
                    // PUT /{Bucket}/{Key}?retention - PutObjectRetention
                    self.handle_put_object_retention(&state, bucket, key, &query, &request)
                        .await
                } else if query.contains_key("renameObject") {
                    // PUT /{Bucket}/{Key}?renameObject - RenameObject
                    self.handle_rename_object(&state, bucket, key, &request)
                        .await
                } else if query.contains_key("encryption") {
                    // PUT /{Bucket}/{Key}?encryption - UpdateObjectEncryption
                    self.handle_update_object_encryption(&state, bucket, key)
                        .await
                } else {
                    let content_type = request
                        .headers
                        .get(http::header::CONTENT_TYPE)
                        .and_then(|v| v.to_str().ok())
                        .map(|s| s.to_string());

                    // Extract x-amz-meta-* headers as user metadata
                    let metadata: Vec<(String, String)> = request
                        .headers
                        .iter()
                        .filter_map(|(name, value)| {
                            let name_str = name.as_str();
                            if name_str.starts_with("x-amz-meta-") {
                                Some((
                                    name_str.strip_prefix("x-amz-meta-").unwrap().to_string(),
                                    value.to_str().unwrap_or("").to_string(),
                                ))
                            } else {
                                None
                            }
                        })
                        .collect();

                    if request.headers.contains_key("x-amz-copy-source") {
                        // PUT /{Bucket}/{Key} with x-amz-copy-source - CopyObject
                        self.handle_copy_object(&blobs, &state, bucket, key, &request, metadata)
                            .await
                    } else {
                        self.handle_put_object(
                            &blobs,
                            &state,
                            bucket,
                            key,
                            request.body,
                            content_type.as_deref(),
                            metadata,
                        )
                        .await
                    }
                }
            }
            (Some(bucket), Some(key), "GET") => {
                if query.contains_key("acl") {
                    // GET /{Bucket}/{Key}?acl - GetObjectAcl
                    self.handle_get_object_acl(&state, bucket, key).await
                } else if query.contains_key("tagging") {
                    // GET /{Bucket}/{Key}?tagging - GetObjectTagging
                    self.handle_get_object_tagging(&state, bucket, key).await
                } else if query.contains_key("legal-hold") {
                    // GET /{Bucket}/{Key}?legal-hold - GetObjectLegalHold
                    self.handle_get_object_legal_hold(&state, bucket, key).await
                } else if query.contains_key("attributes") {
                    // GET /{Bucket}/{Key}?attributes - GetObjectAttributes
                    self.handle_get_object_attributes(&state, bucket, key).await
                } else if query.contains_key("retention") {
                    // GET /{Bucket}/{Key}?retention - GetObjectRetention
                    self.handle_get_object_retention(&state, bucket, key).await
                } else if query.contains_key("uploadId") {
                    // GET /{Bucket}/{Key}?uploadId=Id - ListParts
                    self.handle_list_parts(&state, bucket, key, &query, &request)
                        .await
                } else if query.contains_key("torrent") {
                    // GET /{Bucket}/{Key}?torrent - GetObjectTorrent
                    self.handle_get_object_torrent(&state, bucket, key).await
                } else {
                    self.handle_get_object(&blobs, &state, bucket, key, &query, &request)
                        .await
                }
            }
            (Some(bucket), Some(key), "HEAD") => {
                self.handle_head_object(&state, bucket, key, &query, &request)
                    .await
            }
            (Some(bucket), Some(key), "DELETE") => {
                if query.contains_key("uploadId") {
                    // DELETE /{Bucket}/{Key}?uploadId=Id - AbortMultipartUpload
                    self.handle_abort_multipart_upload(
                        &blobs, &state, bucket, key, &query, &request,
                    )
                    .await
                } else if query.contains_key("tagging") {
                    // DELETE /{Bucket}/{Key}?tagging - DeleteObjectTagging
                    self.handle_delete_object_tagging(&state, bucket, key).await
                } else {
                    self.handle_delete_object(&blobs, &state, bucket, key, &query, &request)
                        .await
                }
            }
            (Some(bucket), Some(key), "POST") => {
                if query.contains_key("uploads") {
                    // POST /{Bucket}/{Key}?uploads - CreateMultipartUpload
                    self.handle_create_multipart_upload(&state, bucket, key, &query, &request)
                        .await
                } else if query.contains_key("uploadId") {
                    // POST /{Bucket}/{Key}?uploadId=Id - CompleteMultipartUpload
                    self.handle_complete_multipart_upload(
                        &blobs, &state, bucket, key, &query, &request,
                    )
                    .await
                } else if query.contains_key("select") {
                    // POST /{Bucket}/{Key}?select&select-type=2 - SelectObjectContent
                    self.handle_select_object_content(&blobs, &state, bucket, key, &query, &request)
                        .await
                } else if query.contains_key("restore") {
                    // POST /{Bucket}/{Key}?restore - RestoreObject
                    self.handle_restore_object(&state, bucket, key).await
                } else {
                    s3_error_response(&S3Error::MethodNotAllowed {
                        method: method.to_string(),
                        resource: request.uri.clone(),
                    })
                }
            }

            _ => s3_error_response(&S3Error::MethodNotAllowed {
                method: method.to_string(),
                resource: request.uri.clone(),
            }),
        };

        if matches!(method, "PUT" | "POST" | "DELETE") && response.status / 100 == 2 {
            self.notify_state_changed(account_id, region).await;
            // Persist bucket-config stored record after any successful mutation that touches
            // bucket-level configuration (i.e. bucket is set, key is not — object and
            // multipart operations are handled at their own call sites).
            if let (Some(bucket), None) = (&target.bucket, &target.key) {
                self.persist_bucket_config(&blobs, &state, bucket).await;
            }
        }
        response
    }

    async fn handle_list_buckets(&self, state: &Arc<tokio::sync::RwLock<S3State>>) -> MockResponse {
        let state = state.read().await;
        let buckets = state.list_buckets();

        let bucket_list: Vec<wire::Bucket> = buckets
            .iter()
            .map(|b| wire::Bucket {
                name: Some(b.name.clone()),
                creation_date: Some(b.creation_date.format("%Y-%m-%dT%H:%M:%S.000Z").to_string()),
                ..Default::default()
            })
            .collect();

        let output = wire::ListBucketsOutput {
            buckets: Some(wire::Buckets::from(bucket_list)),
            owner: Some(wire::Owner {
                i_d: Some(DEFAULT_ACCOUNT_ID.to_string()),
                display_name: Some("winterbaume".to_string()),
            }),
            ..Default::default()
        };

        wire::serialize_list_buckets_response(&output)
    }

    async fn handle_create_bucket(
        &self,
        blobs: &BlobStore,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        region: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.create_bucket(bucket_name, region) {
            Ok(_) => {
                let mut headers = http::HeaderMap::new();
                headers.insert("Location", format!("/{bucket_name}").parse().unwrap());
                headers.insert(
                    http::header::CONTENT_TYPE,
                    "application/xml".parse().unwrap(),
                );
                if let Some(bs) = state.buckets.get(bucket_name) {
                    let meta = crate::stored::StoredBucketConfig::from_state(bucket_name, bs);
                    drop(state);
                    if let Ok(v) = serde_json::to_value(&meta) {
                        let _ = blobs.put_bucket_config(bucket_name, &v).await;
                    }
                } else {
                    drop(state);
                }
                MockResponse {
                    status: 200,
                    headers,
                    body: Bytes::new(),
                }
            }
            Err(e) => s3_error_response(&e),
        }
    }

    async fn handle_head_bucket(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.head_bucket(bucket_name) {
            Ok(bucket) => {
                let mut headers = http::HeaderMap::new();
                headers.insert("x-amz-bucket-region", bucket.region.parse().unwrap());
                MockResponse {
                    status: 200,
                    headers,
                    body: Bytes::new(),
                }
            }
            Err(e) => s3_error_response(&e),
        }
    }

    async fn handle_delete_bucket(
        &self,
        blobs: &BlobStore,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let delete_result = {
            let mut state = state.write().await;
            state.delete_bucket(bucket_name)
        };
        match delete_result {
            Ok(()) => {
                // Clean up any orphaned multipart-upload part blobs for this bucket.
                let _ = blobs.delete_by_prefix(&format!("{bucket_name}/")).await;
                let _ = blobs.delete_bucket_config(bucket_name).await;
                MockResponse {
                    status: 204,
                    headers: http::HeaderMap::new(),
                    body: Bytes::new(),
                }
            }
            Err(e) => s3_error_response(&e),
        }
    }

    async fn handle_put_object(
        &self,
        blobs: &BlobStore,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        key: &str,
        body: Bytes,
        content_type: Option<&str>,
        metadata: Vec<(String, String)>,
    ) -> MockResponse {
        let content_length = body.len() as u64;
        let etag = compute_etag(&body);
        let blob_key = encode_blob_key(bucket_name, key);
        // When versioning is off, remember the old blob_version_id so we can clean it up.
        let old_blob_version_id: Option<String> = {
            let state = state.read().await;
            let versioning_on = state
                .buckets
                .get(bucket_name)
                .map(|bs| bs.versioning_status.as_deref() == Some("Enabled"))
                .unwrap_or(false);
            if !versioning_on {
                state
                    .get_object(bucket_name, key, None)
                    .ok()
                    .map(|obj| obj.blob_version_id.clone())
            } else {
                None
            }
        };
        let blob_version_id = match blobs
            .put_versioned_from(&blob_key, &mut body.as_ref())
            .await
        {
            Ok(vid) => vid,
            Err(e) => {
                return s3_error_response(&S3Error::InternalError {
                    message: e.to_string(),
                    resource: format!("/{bucket_name}/{key}"),
                });
            }
        };
        let mut state = state.write().await;
        match state.put_object(
            bucket_name,
            key,
            blob_key.clone(),
            blob_version_id,
            etag,
            content_length,
            content_type,
            metadata,
        ) {
            Ok(obj) => {
                let mut headers = http::HeaderMap::new();
                headers.insert("ETag", obj.etag.parse().unwrap());
                if obj.version_id != "null" {
                    headers.insert("x-amz-version-id", obj.version_id.parse().unwrap());
                }
                let meta = crate::stored::StoredObjectVersion::from_object(bucket_name, obj);
                let new_bvid = obj.blob_version_id.clone();
                drop(state);
                // Write stored record for the new version.
                if let Ok(v) = serde_json::to_value(&meta) {
                    let _ = blobs.put_version_meta(&blob_key, &new_bvid, &v).await;
                }
                // Delete the superseded blob version and its stored record (only when versioning is off).
                if let Some(old_bvid) = old_blob_version_id {
                    let _ = blobs.delete_version(&blob_key, &old_bvid).await;
                    let _ = blobs.delete_version_meta(&blob_key, &old_bvid).await;
                }
                MockResponse {
                    status: 200,
                    headers,
                    body: Bytes::new(),
                }
            }
            Err(e) => s3_error_response(&e),
        }
    }

    async fn handle_get_object(
        &self,
        blobs: &BlobStore,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        key: &str,
        query: &std::collections::HashMap<String, String>,
        request: &MockRequest,
    ) -> MockResponse {
        let version_id = query.get("versionId").map(String::as_str);
        let obj_info = {
            let state = state.read().await;
            match state.get_object(bucket_name, key, version_id) {
                Ok(obj) => Ok((
                    obj.blob_key.clone(),
                    obj.blob_version_id.clone(),
                    obj.etag.clone(),
                    obj.content_type.clone(),
                    obj.size(),
                    obj.last_modified,
                    obj.metadata.clone(),
                    obj.version_id.clone(),
                )),
                Err(e) => Err(e),
            }
        };
        let _ = request; // currently unused beyond version_id extraction via query
        match obj_info {
            Ok((
                blob_key,
                blob_version_id,
                etag,
                content_type,
                size,
                last_modified,
                metadata,
                vid,
            )) => {
                let body = {
                    let mut reader =
                        match blobs.get_version_reader(&blob_key, &blob_version_id).await {
                            Ok(r) => r,
                            Err(e) => {
                                return s3_error_response(&S3Error::InternalError {
                                    message: e.to_string(),
                                    resource: format!("/{bucket_name}/{key}"),
                                });
                            }
                        };
                    use tokio::io::AsyncReadExt;
                    let mut buf = Vec::new();
                    if let Err(e) = reader.read_to_end(&mut buf).await {
                        return s3_error_response(&S3Error::InternalError {
                            message: e.to_string(),
                            resource: format!("/{bucket_name}/{key}"),
                        });
                    }
                    Bytes::from(buf)
                };
                let mut headers = http::HeaderMap::new();
                headers.insert("ETag", etag.parse().unwrap());
                headers.insert(http::header::CONTENT_TYPE, content_type.parse().unwrap());
                headers.insert(
                    http::header::CONTENT_LENGTH,
                    size.to_string().parse().unwrap(),
                );
                headers.insert(
                    "Last-Modified",
                    last_modified
                        .format("%a, %d %b %Y %H:%M:%S GMT")
                        .to_string()
                        .parse()
                        .unwrap(),
                );
                if vid != "null" {
                    headers.insert("x-amz-version-id", vid.parse().unwrap());
                }
                // Add user metadata headers
                for (k, v) in &metadata {
                    if let (Ok(name), Ok(val)) = (
                        http::header::HeaderName::from_bytes(format!("x-amz-meta-{k}").as_bytes()),
                        http::header::HeaderValue::from_str(v),
                    ) {
                        headers.insert(name, val);
                    }
                }
                MockResponse {
                    status: 200,
                    headers,
                    body,
                }
            }
            Err(e) => s3_error_response(&e),
        }
    }

    async fn handle_head_object(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        key: &str,
        query: &std::collections::HashMap<String, String>,
        _request: &MockRequest,
    ) -> MockResponse {
        let version_id = query.get("versionId").map(String::as_str);
        let state = state.read().await;
        match state.head_object(bucket_name, key, version_id) {
            Ok(obj) => {
                let mut headers = http::HeaderMap::new();
                headers.insert("ETag", obj.etag.parse().unwrap());
                headers.insert(
                    http::header::CONTENT_TYPE,
                    obj.content_type.parse().unwrap(),
                );
                headers.insert(
                    http::header::CONTENT_LENGTH,
                    obj.size().to_string().parse().unwrap(),
                );
                headers.insert(
                    "Last-Modified",
                    obj.last_modified
                        .format("%a, %d %b %Y %H:%M:%S GMT")
                        .to_string()
                        .parse()
                        .unwrap(),
                );
                if obj.version_id != "null" {
                    headers.insert("x-amz-version-id", obj.version_id.parse().unwrap());
                }
                MockResponse {
                    status: 200,
                    headers,
                    body: Bytes::new(),
                }
            }
            Err(e) => s3_error_response(&e),
        }
    }

    async fn handle_delete_object(
        &self,
        blobs: &BlobStore,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        key: &str,
        query: &std::collections::HashMap<String, String>,
        _request: &MockRequest,
    ) -> MockResponse {
        use crate::state::DeleteObjectOutcome;
        let version_id = query.get("versionId").map(String::as_str);
        let delete_result = {
            let mut state = state.write().await;
            state.delete_object(bucket_name, key, version_id)
        };
        match delete_result {
            Ok(outcome) => {
                let mut headers = http::HeaderMap::new();
                match outcome {
                    DeleteObjectOutcome::Deleted {
                        blob_key,
                        blob_version_id,
                    } => {
                        let _ = blobs.delete_version(&blob_key, &blob_version_id).await;
                        let _ = blobs.delete_version_meta(&blob_key, &blob_version_id).await;
                    }
                    DeleteObjectOutcome::DeleteMarkerCreated {
                        version_id: dm_version_id,
                    } => {
                        headers.insert("x-amz-delete-marker", "true".parse().unwrap());
                        headers.insert("x-amz-version-id", dm_version_id.clone().parse().unwrap());
                        // Write stored delete-marker record.
                        let blob_key = encode_blob_key(bucket_name, key);
                        let dm = state
                            .read()
                            .await
                            .buckets
                            .get(bucket_name)
                            .and_then(|bs| bs.delete_markers.get(key))
                            .and_then(|v| v.iter().find(|m| m.version_id == dm_version_id))
                            .cloned();
                        if let Some(dm) = dm {
                            let meta = crate::stored::StoredDeleteMarker::from_marker(
                                bucket_name,
                                &blob_key,
                                &dm,
                            );
                            if let Ok(v) = serde_json::to_value(&meta) {
                                let _ = blobs
                                    .put_delete_marker_meta(&blob_key, &dm_version_id, &v)
                                    .await;
                            }
                        }
                    }
                    DeleteObjectOutcome::NotFound => {}
                }
                MockResponse {
                    status: 204,
                    headers,
                    body: Bytes::new(),
                }
            }
            Err(e) => s3_error_response(&e),
        }
    }

    async fn handle_copy_object(
        &self,
        blobs: &BlobStore,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        key: &str,
        request: &MockRequest,
        metadata: Vec<(String, String)>,
    ) -> MockResponse {
        let copy_source = match request
            .headers
            .get("x-amz-copy-source")
            .and_then(|value| value.to_str().ok())
        {
            Some(value) => percent_decode(value).trim_start_matches('/').to_string(),
            None => {
                return s3_error_response(&S3Error::MissingCopySourceHeader {
                    resource: format!("/{bucket_name}/{key}"),
                });
            }
        };
        let (source_bucket, source_key) = match copy_source.split_once('/') {
            Some(parts) => parts,
            None => {
                return s3_error_response(&S3Error::InvalidCopySourceHeader {
                    resource: format!("/{bucket_name}/{key}"),
                });
            }
        };

        let (
            source_blob_key,
            source_blob_version_id,
            source_etag,
            source_content_length,
            source_content_type,
            source_metadata,
        ) = {
            let state = state.read().await;
            match state.get_object(source_bucket, source_key, None) {
                Ok(object) => (
                    object.blob_key.clone(),
                    object.blob_version_id.clone(),
                    object.etag.clone(),
                    object.content_length,
                    object.content_type.clone(),
                    object.metadata.clone(),
                ),
                Err(err) => return s3_error_response(&err),
            }
        };

        let dest_blob_key = encode_blob_key(bucket_name, key);
        let dest_old_blob_version_id: Option<String> = {
            let state = state.read().await;
            let versioning_on = state
                .buckets
                .get(bucket_name)
                .map(|bs| bs.versioning_status.as_deref() == Some("Enabled"))
                .unwrap_or(false);
            if !versioning_on {
                state
                    .get_object(bucket_name, key, None)
                    .ok()
                    .map(|obj| obj.blob_version_id.clone())
            } else {
                None
            }
        };
        let dest_blob_version_id = {
            let mut reader = match blobs
                .get_version_reader(&source_blob_key, &source_blob_version_id)
                .await
            {
                Ok(r) => r,
                Err(e) => {
                    return s3_error_response(&S3Error::InternalError {
                        message: e.to_string(),
                        resource: format!("/{bucket_name}/{key}"),
                    });
                }
            };
            match blobs.put_versioned_from(&dest_blob_key, &mut reader).await {
                Ok(vid) => vid,
                Err(e) => {
                    return s3_error_response(&S3Error::InternalError {
                        message: e.to_string(),
                        resource: format!("/{bucket_name}/{key}"),
                    });
                }
            }
        };

        let metadata = if metadata.is_empty() {
            source_metadata
        } else {
            metadata
        };
        let mut state = state.write().await;
        match state.put_object(
            bucket_name,
            key,
            dest_blob_key.clone(),
            dest_blob_version_id,
            source_etag,
            source_content_length,
            Some(&source_content_type),
            metadata,
        ) {
            Ok(object) => {
                let version_id = object.version_id.clone();
                let meta = crate::stored::StoredObjectVersion::from_object(bucket_name, object);
                let new_bvid = meta.blob_version_id.clone();
                let result = wire::CopyObjectResult {
                    checksum_c_r_c32: None,
                    checksum_c_r_c32_c: None,
                    checksum_c_r_c64_n_v_m_e: None,
                    checksum_s_h_a1: None,
                    checksum_s_h_a256: None,
                    checksum_type: None,
                    e_tag: Some(meta.etag.clone()),
                    last_modified: Some(
                        meta.last_modified
                            .format("%Y-%m-%dT%H:%M:%S.000Z")
                            .to_string(),
                    ),
                };
                let mut resp = wire::serialize_copy_object_response(&result);
                if version_id != "null" {
                    resp.headers
                        .insert("x-amz-version-id", version_id.parse().unwrap());
                }
                drop(state);
                if let Ok(v) = serde_json::to_value(&meta) {
                    let _ = blobs.put_version_meta(&dest_blob_key, &new_bvid, &v).await;
                }
                if let Some(old_bvid) = dest_old_blob_version_id {
                    let _ = blobs.delete_version(&dest_blob_key, &old_bvid).await;
                    let _ = blobs.delete_version_meta(&dest_blob_key, &old_bvid).await;
                }
                resp
            }
            Err(err) => s3_error_response(&err),
        }
    }

    async fn handle_create_multipart_upload(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        key: &str,
        query: &std::collections::HashMap<String, String>,
        request: &MockRequest,
    ) -> MockResponse {
        let input = match deserialize_s3_request(
            wire::deserialize_create_multipart_upload_request(
                request,
                &[("Bucket", bucket_name), ("Key", key)],
                query,
            ),
            &request.uri,
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let metadata = input
            .metadata
            .unwrap_or_default()
            .into_iter()
            .collect::<Vec<_>>();
        let mut state = state.write().await;
        match state.create_multipart_upload(
            bucket_name,
            key,
            input.content_type.as_deref(),
            metadata,
            input.storage_class.as_deref(),
        ) {
            Ok(upload) => {
                let result = wire::CreateMultipartUploadOutput {
                    bucket: Some(bucket_name.to_string()),
                    key: Some(key.to_string()),
                    upload_id: Some(upload.upload_id),
                };
                wire::serialize_create_multipart_upload_response(&result)
            }
            Err(err) => s3_error_response(&err),
        }
    }

    async fn handle_upload_part(
        &self,
        blobs: &BlobStore,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        key: &str,
        query: &std::collections::HashMap<String, String>,
        request: &MockRequest,
    ) -> MockResponse {
        let input = match deserialize_s3_request(
            wire::deserialize_upload_part_request(
                request,
                &[("Bucket", bucket_name), ("Key", key)],
                query,
            ),
            &request.uri,
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let body = Bytes::from(input.body.unwrap_or_default());
        let content_length = body.len() as u64;
        let etag = compute_etag(&body);
        let blob_key = format!(
            "{bucket_name}/.mpu/{}/{}",
            input.upload_id, input.part_number
        );
        if let Err(e) = blobs.put_from(&blob_key, &mut body.as_ref()).await {
            return s3_error_response(&S3Error::InternalError {
                message: e.to_string(),
                resource: format!("/{bucket_name}/{key}"),
            });
        }
        let mut state = state.write().await;
        match state.upload_part(
            bucket_name,
            key,
            &input.upload_id,
            input.part_number,
            blob_key,
            etag,
            content_length,
        ) {
            Ok(part) => {
                let mut response = wire::serialize_upload_part_response();
                response.headers.insert("ETag", part.etag.parse().unwrap());
                response
            }
            Err(err) => s3_error_response(&err),
        }
    }

    async fn handle_upload_part_copy(
        &self,
        blobs: &BlobStore,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        key: &str,
        query: &std::collections::HashMap<String, String>,
        request: &MockRequest,
    ) -> MockResponse {
        let input = match deserialize_s3_request(
            wire::deserialize_upload_part_copy_request(
                request,
                &[("Bucket", bucket_name), ("Key", key)],
                query,
            ),
            &request.uri,
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let (source_bucket, source_key) =
            match parse_copy_source_header(request, &format!("/{bucket_name}/{key}")) {
                Ok(parts) => parts,
                Err(response) => return response,
            };

        // Stream the source object blob into a new part blob key.
        // etag and content_length come from the already-stored object metadata.
        let (source_blob_key, source_blob_version_id, content_length, etag) = {
            let state = state.read().await;
            match state.get_object(&source_bucket, &source_key, None) {
                Ok(obj) => (
                    obj.blob_key.clone(),
                    obj.blob_version_id.clone(),
                    obj.content_length,
                    obj.etag.clone(),
                ),
                Err(err) => return s3_error_response(&err),
            }
        };
        let part_blob_key = format!(
            "{bucket_name}/.mpu/{}/{}",
            input.upload_id, input.part_number
        );
        {
            let mut reader = match blobs
                .get_version_reader(&source_blob_key, &source_blob_version_id)
                .await
            {
                Ok(r) => r,
                Err(e) => {
                    return s3_error_response(&S3Error::InternalError {
                        message: e.to_string(),
                        resource: format!("/{bucket_name}/{key}"),
                    });
                }
            };
            if let Err(e) = blobs.put_from(&part_blob_key, &mut reader).await {
                return s3_error_response(&S3Error::InternalError {
                    message: e.to_string(),
                    resource: format!("/{bucket_name}/{key}"),
                });
            }
        }

        let mut state = state.write().await;
        match state.upload_part(
            bucket_name,
            key,
            &input.upload_id,
            input.part_number,
            part_blob_key,
            etag,
            content_length,
        ) {
            Ok(part) => {
                let result = wire::CopyPartResult {
                    checksum_c_r_c32: None,
                    checksum_c_r_c32_c: None,
                    checksum_c_r_c64_n_v_m_e: None,
                    checksum_s_h_a1: None,
                    checksum_s_h_a256: None,
                    e_tag: Some(part.etag),
                    last_modified: Some(
                        part.last_modified
                            .format("%Y-%m-%dT%H:%M:%S.000Z")
                            .to_string(),
                    ),
                };
                wire::serialize_upload_part_copy_response(&result)
            }
            Err(err) => s3_error_response(&err),
        }
    }

    async fn handle_complete_multipart_upload(
        &self,
        blobs: &BlobStore,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        key: &str,
        query: &std::collections::HashMap<String, String>,
        request: &MockRequest,
    ) -> MockResponse {
        let input = match deserialize_s3_request(
            wire::deserialize_complete_multipart_upload_request(
                request,
                &[("Bucket", bucket_name), ("Key", key)],
                query,
            ),
            &request.uri,
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let completed_parts = input.multipart_upload.map(|upload| {
            upload
                .parts
                .unwrap_or_default()
                .into_iter()
                .filter_map(|part| {
                    part.part_number
                        .map(|part_number| (part_number, part.e_tag))
                })
                .collect::<Vec<_>>()
        });

        // Step 1: validate parts and collect part blob_keys (read lock).
        let selected_parts = {
            let state = state.read().await;
            match state.select_multipart_parts(
                bucket_name,
                key,
                &input.upload_id,
                completed_parts.as_ref(),
            ) {
                Ok(parts) => parts,
                Err(err) => return s3_error_response(&err),
            }
        };

        // Step 2: compute ETag and content-length from part metadata (no blob reads needed).
        // Real S3 multipart ETag = "{md5(concat(part_etag_bytes))}-{num_parts}".
        let part_etag_bytes: Vec<u8> = selected_parts
            .iter()
            .flat_map(|p| {
                // Part ETags are double-quoted hex strings; decode the inner hex bytes.
                let inner = p.etag.trim_matches('"');
                (0..inner.len())
                    .step_by(2)
                    .filter_map(|i| u8::from_str_radix(&inner[i..i + 2], 16).ok())
                    .collect::<Vec<u8>>()
            })
            .collect();
        let final_etag = format!(
            "\"{}-{}\"",
            compute_etag_raw(&part_etag_bytes),
            selected_parts.len()
        );
        let final_content_length: u64 = selected_parts.iter().map(|p| p.content_length).sum();

        // Step 3: assemble parts into a versioned blob under the deterministic blob_key.
        let final_blob_key = encode_blob_key(bucket_name, key);
        let old_blob_version_id: Option<String> = {
            let state = state.read().await;
            let versioning_on = state
                .buckets
                .get(bucket_name)
                .map(|bs| bs.versioning_status.as_deref() == Some("Enabled"))
                .unwrap_or(false);
            if !versioning_on {
                state
                    .get_object(bucket_name, key, None)
                    .ok()
                    .map(|obj| obj.blob_version_id.clone())
            } else {
                None
            }
        };
        let part_blob_keys: Vec<String> =
            selected_parts.iter().map(|p| p.blob_key.clone()).collect();
        // Assemble parts eagerly into a single buffer, then store as a new versioned blob.
        let mut assembled = bytes::BytesMut::new();
        for part_blob_key in &part_blob_keys {
            let part_data = match blobs.get(part_blob_key).await {
                Ok(d) => d,
                Err(e) => {
                    return s3_error_response(&S3Error::InternalError {
                        message: e.to_string(),
                        resource: format!("/{bucket_name}/{key}"),
                    });
                }
            };
            assembled.extend_from_slice(&part_data);
        }
        let final_blob_version_id = match blobs
            .put_versioned_from(&final_blob_key, &mut assembled.freeze().as_ref())
            .await
        {
            Ok(vid) => vid,
            Err(e) => {
                return s3_error_response(&S3Error::InternalError {
                    message: e.to_string(),
                    resource: format!("/{bucket_name}/{key}"),
                });
            }
        };
        // Clean up the now-assembled part blobs.
        for part_blob_key in part_blob_keys {
            let _ = blobs.delete(&part_blob_key).await;
        }

        // Step 4: finalize in state (write lock).
        let mut state_guard = state.write().await;
        match state_guard.finalize_multipart_upload(
            bucket_name,
            key,
            &input.upload_id,
            final_blob_key.clone(),
            final_blob_version_id,
            final_etag,
            final_content_length,
        ) {
            Ok(object) => {
                let version_id = object.version_id.clone();
                let meta = crate::stored::StoredObjectVersion::from_object(bucket_name, &object);
                let new_bvid = meta.blob_version_id.clone();
                let etag = object.etag.clone();
                drop(state_guard);
                if let Ok(v) = serde_json::to_value(&meta) {
                    let _ = blobs.put_version_meta(&final_blob_key, &new_bvid, &v).await;
                }
                if let Some(old_bvid) = old_blob_version_id {
                    let _ = blobs.delete_version(&final_blob_key, &old_bvid).await;
                    let _ = blobs.delete_version_meta(&final_blob_key, &old_bvid).await;
                }
                let result = wire::CompleteMultipartUploadOutput {
                    bucket: Some(bucket_name.to_string()),
                    checksum_c_r_c32: None,
                    checksum_c_r_c32_c: None,
                    checksum_c_r_c64_n_v_m_e: None,
                    checksum_s_h_a1: None,
                    checksum_s_h_a256: None,
                    checksum_type: None,
                    e_tag: Some(etag),
                    key: Some(key.to_string()),
                    location: Some(format!("/{bucket_name}/{key}")),
                };
                let mut resp = wire::serialize_complete_multipart_upload_response(&result);
                if version_id != "null" {
                    resp.headers
                        .insert("x-amz-version-id", version_id.parse().unwrap());
                }
                resp
            }
            Err(err) => s3_error_response(&err),
        }
    }

    async fn handle_abort_multipart_upload(
        &self,
        blobs: &BlobStore,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        key: &str,
        query: &std::collections::HashMap<String, String>,
        request: &MockRequest,
    ) -> MockResponse {
        let input = match deserialize_s3_request(
            wire::deserialize_abort_multipart_upload_request(
                request,
                &[("Bucket", bucket_name), ("Key", key)],
                query,
            ),
            &request.uri,
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let abort_result = {
            let mut state_guard = state.write().await;
            state_guard.abort_multipart_upload(bucket_name, key, &input.upload_id)
        };
        match abort_result {
            Ok(part_blob_keys) => {
                for blob_key in part_blob_keys {
                    let _ = blobs.delete(&blob_key).await;
                }
                wire::serialize_abort_multipart_upload_response()
            }
            Err(err) => s3_error_response(&err),
        }
    }

    async fn handle_list_multipart_uploads(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        query: &std::collections::HashMap<String, String>,
        request: &MockRequest,
    ) -> MockResponse {
        let input = match deserialize_s3_request(
            wire::deserialize_list_multipart_uploads_request(
                request,
                &[("Bucket", bucket_name)],
                query,
            ),
            &request.uri,
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let state = state.read().await;
        match state.list_multipart_uploads(bucket_name, input.prefix.as_deref()) {
            Ok(uploads) => {
                let result = wire::ListMultipartUploadsOutput {
                    bucket: Some(bucket_name.to_string()),
                    common_prefixes: None,
                    delimiter: input.delimiter,
                    encoding_type: input.encoding_type,
                    is_truncated: Some(false),
                    key_marker: input.key_marker,
                    max_uploads: input.max_uploads,
                    next_key_marker: None,
                    next_upload_id_marker: None,
                    prefix: input.prefix,
                    upload_id_marker: input.upload_id_marker,
                    uploads: (!uploads.is_empty()).then(|| {
                        uploads
                            .into_iter()
                            .map(|upload| wire::MultipartUpload {
                                checksum_algorithm: None,
                                checksum_type: None,
                                initiated: Some(
                                    upload
                                        .initiated
                                        .format("%Y-%m-%dT%H:%M:%S.000Z")
                                        .to_string(),
                                ),
                                initiator: Some(default_initiator()),
                                key: Some(upload.key),
                                owner: Some(default_owner()),
                                storage_class: Some(upload.storage_class),
                                upload_id: Some(upload.upload_id),
                            })
                            .collect()
                    }),
                };
                wire::serialize_list_multipart_uploads_response(&result)
            }
            Err(err) => s3_error_response(&err),
        }
    }

    async fn handle_list_parts(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        key: &str,
        query: &std::collections::HashMap<String, String>,
        request: &MockRequest,
    ) -> MockResponse {
        let input = match deserialize_s3_request(
            wire::deserialize_list_parts_request(
                request,
                &[("Bucket", bucket_name), ("Key", key)],
                query,
            ),
            &request.uri,
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let state = state.read().await;
        match state.list_parts(bucket_name, key, &input.upload_id) {
            Ok(parts) => {
                let result = wire::ListPartsOutput {
                    bucket: Some(bucket_name.to_string()),
                    checksum_algorithm: None,
                    checksum_type: None,
                    initiator: Some(default_initiator()),
                    is_truncated: Some(false),
                    key: Some(key.to_string()),
                    max_parts: input.max_parts,
                    next_part_number_marker: None,
                    owner: Some(default_owner()),
                    part_number_marker: input.part_number_marker,
                    parts: (!parts.is_empty()).then(|| {
                        parts
                            .into_iter()
                            .map(|part| wire::Part {
                                checksum_c_r_c32: None,
                                checksum_c_r_c32_c: None,
                                checksum_c_r_c64_n_v_m_e: None,
                                checksum_s_h_a1: None,
                                checksum_s_h_a256: None,
                                e_tag: Some(part.etag),
                                last_modified: Some(
                                    part.last_modified
                                        .format("%Y-%m-%dT%H:%M:%S.000Z")
                                        .to_string(),
                                ),
                                part_number: Some(part.part_number),
                                size: Some(part.content_length as i64),
                            })
                            .collect()
                    }),
                    storage_class: Some("STANDARD".to_string()),
                    upload_id: Some(input.upload_id),
                };
                wire::serialize_list_parts_response(&result)
            }
            Err(err) => s3_error_response(&err),
        }
    }

    async fn handle_list_object_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        query: &std::collections::HashMap<String, String>,
        request: &MockRequest,
    ) -> MockResponse {
        let input = match deserialize_s3_request(
            wire::deserialize_list_object_versions_request(
                request,
                &[("Bucket", bucket_name)],
                query,
            ),
            &request.uri,
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let max_keys = input.max_keys.unwrap_or(1000).max(0) as usize;
        let state = state.read().await;
        match state.list_object_versions(
            bucket_name,
            input.prefix.as_deref(),
            input.delimiter.as_deref(),
            max_keys,
        ) {
            Ok((version_entries, delete_marker_entries, common_prefixes, is_truncated)) => {
                let versions = version_entries
                    .iter()
                    .map(|(object, is_latest)| wire::ObjectVersion {
                        checksum_algorithm: None,
                        checksum_type: None,
                        e_tag: Some(object.etag.clone()),
                        is_latest: Some(*is_latest),
                        key: Some(object.key.clone()),
                        last_modified: Some(
                            object
                                .last_modified
                                .format("%Y-%m-%dT%H:%M:%S.000Z")
                                .to_string(),
                        ),
                        owner: Some(default_owner()),
                        restore_status: None,
                        size: Some(object.size() as i64),
                        storage_class: Some(object.storage_class.clone()),
                        version_id: Some(object.version_id.clone()),
                    })
                    .collect::<Vec<_>>();
                let wire_delete_markers = delete_marker_entries
                    .iter()
                    .map(|dm| wire::DeleteMarkerEntry {
                        is_latest: Some(true),
                        key: Some(dm.key.clone()),
                        last_modified: Some(
                            dm.last_modified
                                .format("%Y-%m-%dT%H:%M:%S.000Z")
                                .to_string(),
                        ),
                        owner: Some(default_owner()),
                        version_id: Some(dm.version_id.clone()),
                    })
                    .collect::<Vec<_>>();
                let result = wire::ListObjectVersionsOutput {
                    common_prefixes: (!common_prefixes.is_empty()).then(|| {
                        common_prefixes
                            .into_iter()
                            .map(|prefix| wire::CommonPrefix {
                                prefix: Some(prefix.prefix),
                            })
                            .collect()
                    }),
                    delete_markers: (!wire_delete_markers.is_empty())
                        .then_some(wire_delete_markers),
                    delimiter: input.delimiter,
                    encoding_type: input.encoding_type,
                    is_truncated: Some(is_truncated),
                    key_marker: input.key_marker,
                    max_keys: input.max_keys,
                    name: Some(bucket_name.to_string()),
                    next_key_marker: None,
                    next_version_id_marker: None,
                    prefix: input.prefix,
                    version_id_marker: input.version_id_marker,
                    versions: (!versions.is_empty()).then_some(versions),
                };
                wire::serialize_list_object_versions_response(&result)
            }
            Err(err) => s3_error_response(&err),
        }
    }

    async fn handle_select_object_content(
        &self,
        blobs: &BlobStore,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        key: &str,
        query: &std::collections::HashMap<String, String>,
        request: &MockRequest,
    ) -> MockResponse {
        if let Err(response) = deserialize_s3_request(
            wire::deserialize_select_object_content_request(
                request,
                &[("Bucket", bucket_name), ("Key", key)],
                query,
            ),
            &request.uri,
        ) {
            return response;
        }
        let blob_key_result = {
            let state = state.read().await;
            state
                .get_object(bucket_name, key, None)
                .map(|obj| (obj.blob_key.clone(), obj.blob_version_id.clone()))
        };
        match blob_key_result {
            Ok((blob_key, blob_version_id)) => {
                let body = {
                    let mut reader =
                        match blobs.get_version_reader(&blob_key, &blob_version_id).await {
                            Ok(r) => r,
                            Err(e) => {
                                return s3_error_response(&S3Error::InternalError {
                                    message: e.to_string(),
                                    resource: format!("/{bucket_name}/{key}"),
                                });
                            }
                        };
                    let mut buf = Vec::new();
                    if let Err(e) = reader.read_to_end(&mut buf).await {
                        return s3_error_response(&S3Error::InternalError {
                            message: e.to_string(),
                            resource: format!("/{bucket_name}/{key}"),
                        });
                    }
                    Bytes::from(buf)
                };
                let result = wire::SelectObjectContentEventStream {
                    cont: None,
                    end: Some(wire::EndEvent {}),
                    progress: None,
                    records: Some(wire::RecordsEvent {
                        payload: Some(String::from_utf8_lossy(&body).into_owned()),
                    }),
                    stats: None,
                };
                wire::serialize_select_object_content_response(&result)
            }
            Err(err) => s3_error_response(&err),
        }
    }

    async fn handle_delete_objects(
        &self,
        blobs: &BlobStore,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        query: &std::collections::HashMap<String, String>,
        request: &MockRequest,
    ) -> MockResponse {
        let input = match deserialize_s3_request(
            wire::deserialize_delete_objects_request(request, &[("Bucket", bucket_name)], query),
            &request.uri,
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let (deleted, blobs_to_delete, dm_stored) = {
            let mut state = state.write().await;

            if !state.buckets.contains_key(bucket_name) {
                return s3_error_response(&no_such_bucket(bucket_name));
            }

            use crate::state::DeleteObjectOutcome;
            let mut deleted = Vec::new();
            let mut blobs_to_delete = Vec::new();
            // (blob_key, dm_version_id, object_key) for delete-marker stored record writes.
            let mut dm_stored: Vec<(String, String, String)> = Vec::new();
            for object in input.delete.objects {
                let version_id_ref = object.version_id.as_deref();
                let mut delete_marker = None;
                let mut delete_marker_version_id = None;
                let mut result_version_id = object.version_id.clone();
                match state.delete_object(bucket_name, &object.key, version_id_ref) {
                    Ok(DeleteObjectOutcome::Deleted {
                        blob_key,
                        blob_version_id,
                    }) => {
                        blobs_to_delete.push((blob_key, blob_version_id));
                    }
                    Ok(DeleteObjectOutcome::DeleteMarkerCreated { version_id }) => {
                        delete_marker = Some(true);
                        delete_marker_version_id = Some(version_id.clone());
                        result_version_id = Some(version_id.clone());
                        dm_stored.push((
                            encode_blob_key(bucket_name, &object.key),
                            version_id,
                            object.key.clone(),
                        ));
                    }
                    Ok(DeleteObjectOutcome::NotFound) => {}
                    Err(_) => {}
                }
                deleted.push(wire::DeletedObject {
                    delete_marker,
                    delete_marker_version_id,
                    key: Some(object.key),
                    version_id: result_version_id,
                });
            }
            (deleted, blobs_to_delete, dm_stored)
        };
        for (blob_key, blob_version_id) in blobs_to_delete {
            let _ = blobs.delete_version(&blob_key, &blob_version_id).await;
            let _ = blobs.delete_version_meta(&blob_key, &blob_version_id).await;
        }
        // Write stored delete-marker records.
        for (blob_key, dm_version_id, obj_key) in dm_stored {
            let dm = state
                .read()
                .await
                .buckets
                .get(bucket_name)
                .and_then(|bs| bs.delete_markers.get(&obj_key))
                .and_then(|v| v.iter().find(|m| m.version_id == dm_version_id))
                .cloned();
            if let Some(dm) = dm {
                let meta =
                    crate::stored::StoredDeleteMarker::from_marker(bucket_name, &blob_key, &dm);
                if let Ok(v) = serde_json::to_value(&meta) {
                    let _ = blobs
                        .put_delete_marker_meta(&blob_key, &dm_version_id, &v)
                        .await;
                }
            }
        }

        let result = wire::DeleteObjectsOutput {
            deleted: (input.delete.quiet != Some(true)).then_some(deleted),
            errors: None,
        };
        wire::serialize_delete_objects_response(&result)
    }

    async fn handle_list_objects(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        query: &std::collections::HashMap<String, String>,
    ) -> MockResponse {
        let prefix = query.get("prefix").map(String::as_str);
        let delimiter = query.get("delimiter").map(String::as_str);
        let max_keys: usize = query
            .get("max-keys")
            .and_then(|s| s.parse().ok())
            .unwrap_or(1000);
        let marker = query.get("marker").map(String::as_str);

        let state = state.read().await;
        match state.list_objects_v2(bucket_name, prefix, delimiter, max_keys, None, marker) {
            Ok(result) => {
                let output = wire::ListObjectsOutput {
                    common_prefixes: (!result.common_prefixes.is_empty()).then(|| {
                        result
                            .common_prefixes
                            .iter()
                            .map(|prefix| wire::CommonPrefix {
                                prefix: Some(prefix.prefix.clone()),
                            })
                            .collect()
                    }),
                    contents: (!result.objects.is_empty())
                        .then(|| result.objects.iter().map(to_list_object).collect()),
                    delimiter: delimiter.map(str::to_string),
                    encoding_type: None,
                    is_truncated: Some(result.is_truncated),
                    marker: marker.map(str::to_string),
                    max_keys: Some(max_keys as i32),
                    name: Some(bucket_name.to_string()),
                    next_marker: result.next_continuation_token,
                    prefix: prefix.map(str::to_string),
                };
                wire::serialize_list_objects_response(&output)
            }
            Err(err) => s3_error_response(&err),
        }
    }

    async fn handle_list_objects_v2(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        query: &std::collections::HashMap<String, String>,
    ) -> MockResponse {
        let prefix = query.get("prefix").map(|s| s.as_str());
        let delimiter = query.get("delimiter").map(|s| s.as_str());
        let max_keys: usize = query
            .get("max-keys")
            .and_then(|s| s.parse().ok())
            .unwrap_or(1000);
        let continuation_token = query.get("continuation-token").map(|s| s.as_str());
        let start_after = query.get("start-after").map(|s| s.as_str());

        let state = state.read().await;
        match state.list_objects_v2(
            bucket_name,
            prefix,
            delimiter,
            max_keys,
            continuation_token,
            start_after,
        ) {
            Ok(result) => {
                let contents: Vec<wire::Object> = result
                    .objects
                    .iter()
                    .map(|obj| wire::Object {
                        key: Some(obj.key.clone()),
                        last_modified: Some(
                            obj.last_modified
                                .format("%Y-%m-%dT%H:%M:%S.000Z")
                                .to_string(),
                        ),
                        e_tag: Some(obj.etag.clone()),
                        size: Some(obj.size() as i64),
                        storage_class: Some(obj.storage_class.clone()),
                        ..Default::default()
                    })
                    .collect();
                let common_prefixes: Vec<wire::CommonPrefix> = result
                    .common_prefixes
                    .iter()
                    .map(|cp| wire::CommonPrefix {
                        prefix: Some(cp.prefix.clone()),
                    })
                    .collect();
                let output = wire::ListObjectsV2Output {
                    name: Some(bucket_name.to_string()),
                    prefix: Some(prefix.unwrap_or("").to_string()),
                    key_count: Some(result.key_count as i32),
                    max_keys: Some(max_keys as i32),
                    is_truncated: Some(result.is_truncated),
                    next_continuation_token: result.next_continuation_token,
                    contents: (!contents.is_empty()).then_some(contents),
                    common_prefixes: (!common_prefixes.is_empty()).then_some(common_prefixes),
                    ..Default::default()
                };
                wire::serialize_list_objects_v2_response(&output)
            }
            Err(e) => s3_error_response(&e),
        }
    }

    async fn handle_get_bucket_location(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.head_bucket(bucket_name) {
            Ok(bucket) => {
                let result = wire::GetBucketLocationOutput {
                    location_constraint: (bucket.region != "us-east-1")
                        .then(|| bucket.region.clone()),
                };
                wire::serialize_get_bucket_location_response(&result)
            }
            Err(err) => s3_error_response(&err),
        }
    }

    async fn handle_put_bucket_inventory_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        query: &std::collections::HashMap<String, String>,
        request: &MockRequest,
    ) -> MockResponse {
        let input = match deserialize_s3_request(
            wire::deserialize_put_bucket_inventory_configuration_request(
                request,
                &[("Bucket", bucket_name)],
                query,
            ),
            &request.uri,
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        if input.id.is_empty() {
            return s3_error_response(&S3Error::MissingInventoryConfigurationId {
                resource: format!("/{bucket_name}?inventory"),
            });
        }
        let mut state = state.write().await;
        let Some(bucket) = state.buckets.get_mut(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        bucket.inventory_configurations.insert(
            input.id.clone(),
            quick_xml::se::to_string(&input.inventory_configuration).unwrap_or_default(),
        );
        wire::serialize_put_bucket_inventory_configuration_response()
    }

    async fn handle_get_bucket_inventory_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        query: &std::collections::HashMap<String, String>,
    ) -> MockResponse {
        let id = match query.get("id") {
            Some(value) => value,
            None => {
                return s3_error_response(&S3Error::MissingInventoryConfigurationId {
                    resource: format!("/{bucket_name}?inventory"),
                });
            }
        };
        let state = state.read().await;
        let Some(bucket) = state.buckets.get(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        match bucket.inventory_configurations.get(id) {
            Some(xml) => MockResponse::xml(200, xml.clone()),
            None => s3_error_response(&S3Error::NoSuchInventoryConfiguration {
                resource: format!("/{bucket_name}?inventory&id={id}"),
            }),
        }
    }

    async fn handle_list_bucket_inventory_configurations(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let Some(bucket) = state.buckets.get(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        let configurations = bucket
            .inventory_configurations
            .values()
            .filter_map(|xml| quick_xml::de::from_str::<wire::InventoryConfiguration>(xml).ok())
            .collect::<Vec<_>>();
        let result = wire::ListBucketInventoryConfigurationsOutput {
            continuation_token: None,
            inventory_configuration_list: (!configurations.is_empty()).then_some(configurations),
            is_truncated: Some(false),
            next_continuation_token: None,
        };
        wire::serialize_list_bucket_inventory_configurations_response(&result)
    }
    async fn handle_put_bucket_lifecycle_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        body: Bytes,
    ) -> MockResponse {
        let xml = String::from_utf8_lossy(&body).to_string();
        let mut state = state.write().await;
        match state.put_bucket_lifecycle_configuration(bucket_name, xml) {
            Ok(()) => MockResponse {
                status: 200,
                headers: http::HeaderMap::new(),
                body: Bytes::new(),
            },
            Err(e) => s3_error_response(&e),
        }
    }

    async fn handle_get_bucket_lifecycle_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_bucket_lifecycle_configuration(bucket_name) {
            Ok(Some(xml)) => MockResponse::xml(200, xml.to_string()),
            Ok(None) => s3_error_response(&S3Error::NoSuchLifecycleConfiguration {
                resource: format!("/{bucket_name}?lifecycle"),
            }),
            Err(e) => s3_error_response(&e),
        }
    }

    async fn handle_put_bucket_notification_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        body: Bytes,
    ) -> MockResponse {
        let xml = String::from_utf8_lossy(&body).to_string();
        let mut state = state.write().await;
        match state.put_bucket_notification_configuration(bucket_name, xml) {
            Ok(()) => MockResponse {
                status: 200,
                headers: http::HeaderMap::new(),
                body: Bytes::new(),
            },
            Err(e) => s3_error_response(&e),
        }
    }

    async fn handle_get_bucket_notification_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_bucket_notification_configuration(bucket_name) {
            Ok(Some(xml)) => MockResponse::xml(200, xml.to_string()),
            Ok(None) => {
                // AWS returns an empty NotificationConfiguration when none is set
                wire::serialize_get_bucket_notification_configuration_response(
                    &wire::NotificationConfiguration::default(),
                )
            }
            Err(e) => s3_error_response(&e),
        }
    }

    // -----------------------------------------------------------------------
    // Versioning
    // -----------------------------------------------------------------------

    async fn handle_get_bucket_versioning(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let bs = match state.buckets.get(bucket_name) {
            Some(bs) => bs,
            None => return s3_error_response(&no_such_bucket(bucket_name)),
        };
        let output = wire::GetBucketVersioningOutput {
            status: bs.versioning_status.clone(),
            ..Default::default()
        };
        wire::serialize_get_bucket_versioning_response(&output)
    }

    async fn handle_put_bucket_versioning(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        body: Bytes,
    ) -> MockResponse {
        let mut state = state.write().await;
        let bs = match state.buckets.get_mut(bucket_name) {
            Some(bs) => bs,
            None => return s3_error_response(&no_such_bucket(bucket_name)),
        };
        let body_str = String::from_utf8_lossy(&body);
        if body_str.contains("<Status>Enabled</Status>") {
            bs.versioning_status = Some("Enabled".to_string());
        } else if body_str.contains("<Status>Suspended</Status>") {
            bs.versioning_status = Some("Suspended".to_string());
        }
        MockResponse {
            status: 200,
            headers: http::HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    // -----------------------------------------------------------------------
    // ACL
    // -----------------------------------------------------------------------

    async fn handle_get_bucket_acl(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        if !state.buckets.contains_key(bucket_name) {
            return s3_error_response(&no_such_bucket(bucket_name));
        }
        let result = wire::GetBucketAclOutput {
            grants: Some(wire::Grants {
                items: vec![owner_full_control_grant()],
            }),
            owner: Some(default_owner()),
        };
        wire::serialize_get_bucket_acl_response(&result)
    }

    async fn handle_put_bucket_acl(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        body: Bytes,
    ) -> MockResponse {
        let mut state = state.write().await;
        let bs = match state.buckets.get_mut(bucket_name) {
            Some(bs) => bs,
            None => return s3_error_response(&no_such_bucket(bucket_name)),
        };
        bs.acl = Some(String::from_utf8_lossy(&body).to_string());
        MockResponse {
            status: 200,
            headers: http::HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    async fn handle_get_object_acl(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        key: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let object = match state.get_object(bucket_name, key, None) {
            Ok(object) => object,
            Err(err) => return s3_error_response(&err),
        };
        let mut grants = vec![owner_full_control_grant()];
        if object.acl.as_deref() == Some("public-read") {
            grants.push(public_read_grant());
        }
        let result = wire::GetObjectAclOutput {
            grants: Some(wire::Grants { items: grants }),
            owner: Some(default_owner()),
        };
        wire::serialize_get_object_acl_response(&result)
    }

    async fn handle_put_object_acl(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        key: &str,
        request: &MockRequest,
    ) -> MockResponse {
        let acl = request
            .headers
            .get("x-amz-acl")
            .and_then(|value| value.to_str().ok())
            .unwrap_or("private")
            .to_string();
        let mut state = state.write().await;
        let object = match state
            .buckets
            .get_mut(bucket_name)
            .and_then(|bucket| bucket.objects.get_mut(key))
        {
            Some(object) => object,
            None => {
                let err = state
                    .get_object(bucket_name, key, None)
                    .err()
                    .unwrap_or_else(|| no_such_bucket(bucket_name));
                return s3_error_response(&err);
            }
        };
        object.acl = Some(acl);
        wire::serialize_put_object_acl_response()
    }

    async fn handle_get_object_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        key: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_object(bucket_name, key, None) {
            Ok(object) => {
                let result = wire::GetObjectAttributesOutput {
                    checksum: None,
                    e_tag: Some(object.etag.clone()),
                    object_parts: None,
                    object_size: Some(object.size() as i64),
                    storage_class: Some(object.storage_class.clone()),
                };
                wire::serialize_get_object_attributes_response(&result)
            }
            Err(err) => s3_error_response(&err),
        }
    }

    async fn handle_get_object_legal_hold(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        key: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_object(bucket_name, key, None) {
            Ok(object) => {
                wire::serialize_get_object_legal_hold_response(&wire::ObjectLockLegalHold {
                    status: object.legal_hold_status.clone(),
                })
            }
            Err(err) => s3_error_response(&err),
        }
    }

    async fn handle_put_object_legal_hold(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        key: &str,
        query: &std::collections::HashMap<String, String>,
        request: &MockRequest,
    ) -> MockResponse {
        let input = match deserialize_s3_request(
            wire::deserialize_put_object_legal_hold_request(
                request,
                &[("Bucket", bucket_name), ("Key", key)],
                query,
            ),
            &request.uri,
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let status = input.legal_hold.and_then(|legal_hold| legal_hold.status);
        let mut state = state.write().await;
        let object = match state
            .buckets
            .get_mut(bucket_name)
            .and_then(|bucket| bucket.objects.get_mut(key))
        {
            Some(object) => object,
            None => {
                let err = state
                    .get_object(bucket_name, key, None)
                    .err()
                    .unwrap_or_else(|| no_such_bucket(bucket_name));
                return s3_error_response(&err);
            }
        };
        object.legal_hold_status = status;
        wire::serialize_put_object_legal_hold_response()
    }

    async fn handle_get_object_retention(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        key: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_object(bucket_name, key, None) {
            Ok(object) => {
                wire::serialize_get_object_retention_response(&wire::ObjectLockRetention {
                    mode: object.retention_mode.clone(),
                    retain_until_date: object.retain_until_date.clone(),
                })
            }
            Err(err) => s3_error_response(&err),
        }
    }

    async fn handle_put_object_retention(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        key: &str,
        query: &std::collections::HashMap<String, String>,
        request: &MockRequest,
    ) -> MockResponse {
        let input = match deserialize_s3_request(
            wire::deserialize_put_object_retention_request(
                request,
                &[("Bucket", bucket_name), ("Key", key)],
                query,
            ),
            &request.uri,
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let retention = input.retention;
        let mut state = state.write().await;
        let object = match state
            .buckets
            .get_mut(bucket_name)
            .and_then(|bucket| bucket.objects.get_mut(key))
        {
            Some(object) => object,
            None => {
                let err = state
                    .get_object(bucket_name, key, None)
                    .err()
                    .unwrap_or_else(|| no_such_bucket(bucket_name));
                return s3_error_response(&err);
            }
        };
        object.retention_mode = retention
            .as_ref()
            .and_then(|retention| retention.mode.clone());
        object.retain_until_date = retention.and_then(|retention| retention.retain_until_date);
        wire::serialize_put_object_retention_response()
    }

    // STUB[no-engine]: RestoreObject initiates a Glacier tier-change that requires a real
    //   lifecycle engine; the mock validates the object exists and returns 200 with no
    //   state change.
    async fn handle_restore_object(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        key: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_object(bucket_name, key, None) {
            Ok(_) => wire::serialize_restore_object_response(),
            Err(err) => s3_error_response(&err),
        }
    }

    // -----------------------------------------------------------------------
    // Policy
    // -----------------------------------------------------------------------

    async fn handle_get_bucket_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let bs = match state.buckets.get(bucket_name) {
            Some(bs) => bs,
            None => return s3_error_response(&no_such_bucket(bucket_name)),
        };
        match &bs.policy {
            Some(policy) => MockResponse::json(200, policy.clone()),
            None => s3_error_response(&S3Error::NoSuchBucketPolicy {
                resource: format!("/{bucket_name}?policy"),
            }),
        }
    }

    async fn handle_put_bucket_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        body: Bytes,
    ) -> MockResponse {
        let mut state = state.write().await;
        let bs = match state.buckets.get_mut(bucket_name) {
            Some(bs) => bs,
            None => return s3_error_response(&no_such_bucket(bucket_name)),
        };
        bs.policy = Some(String::from_utf8_lossy(&body).to_string());
        MockResponse {
            status: 204,
            headers: http::HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    async fn handle_delete_bucket_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        let bs = match state.buckets.get_mut(bucket_name) {
            Some(bs) => bs,
            None => return s3_error_response(&no_such_bucket(bucket_name)),
        };
        bs.policy = None;
        MockResponse {
            status: 204,
            headers: http::HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    // -----------------------------------------------------------------------
    // Tagging
    // -----------------------------------------------------------------------

    async fn handle_get_bucket_tagging(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let bs = match state.buckets.get(bucket_name) {
            Some(bs) => bs,
            None => return s3_error_response(&no_such_bucket(bucket_name)),
        };
        if bs.tags.is_empty() {
            return s3_error_response(&S3Error::NoSuchTagSet {
                resource: format!("/{bucket_name}?tagging"),
            });
        }
        let mut tags = bs
            .tags
            .iter()
            .map(|(key, value)| wire::Tag {
                key: key.clone(),
                value: value.clone(),
            })
            .collect::<Vec<_>>();
        tags.sort_by(|left, right| left.key.cmp(&right.key));
        let result = wire::GetBucketTaggingOutput {
            tag_set: Some(wire::TagSet { items: tags }),
        };
        wire::serialize_get_bucket_tagging_response(&result)
    }

    async fn handle_put_bucket_tagging(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        query: &std::collections::HashMap<String, String>,
        request: &MockRequest,
    ) -> MockResponse {
        let input = match deserialize_s3_request(
            wire::deserialize_put_bucket_tagging_request(
                request,
                &[("Bucket", bucket_name)],
                query,
            ),
            &request.uri,
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let mut state = state.write().await;
        let bs = match state.buckets.get_mut(bucket_name) {
            Some(bs) => bs,
            None => return s3_error_response(&no_such_bucket(bucket_name)),
        };
        bs.tags.clear();
        for tag in input.tagging.tag_set.items {
            bs.tags.insert(tag.key, tag.value);
        }
        MockResponse {
            status: 204,
            headers: http::HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    async fn handle_delete_bucket_tagging(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        let bs = match state.buckets.get_mut(bucket_name) {
            Some(bs) => bs,
            None => return s3_error_response(&no_such_bucket(bucket_name)),
        };
        bs.tags.clear();
        MockResponse {
            status: 204,
            headers: http::HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    async fn handle_get_object_tagging(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        key: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_object(bucket_name, key, None) {
            Ok(object) => {
                let tags = object
                    .tags
                    .iter()
                    .map(|(key, value)| wire::Tag {
                        key: key.clone(),
                        value: value.clone(),
                    })
                    .collect();
                let result = wire::GetObjectTaggingOutput {
                    tag_set: Some(wire::TagSet { items: tags }),
                };
                wire::serialize_get_object_tagging_response(&result)
            }
            Err(err) => s3_error_response(&err),
        }
    }

    async fn handle_put_object_tagging(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        key: &str,
        query: &std::collections::HashMap<String, String>,
        request: &MockRequest,
    ) -> MockResponse {
        let input = match deserialize_s3_request(
            wire::deserialize_put_object_tagging_request(
                request,
                &[("Bucket", bucket_name), ("Key", key)],
                query,
            ),
            &request.uri,
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let tags = input
            .tagging
            .tag_set
            .items
            .into_iter()
            .map(|tag| (tag.key, tag.value))
            .collect();
        let mut state = state.write().await;
        let object = match state
            .buckets
            .get_mut(bucket_name)
            .and_then(|bucket| bucket.objects.get_mut(key))
        {
            Some(object) => object,
            None => {
                let err = state
                    .get_object(bucket_name, key, None)
                    .err()
                    .unwrap_or_else(|| no_such_bucket(bucket_name));
                return s3_error_response(&err);
            }
        };
        object.tags = tags;
        wire::serialize_put_object_tagging_response()
    }

    async fn handle_delete_object_tagging(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        key: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        let object = match state
            .buckets
            .get_mut(bucket_name)
            .and_then(|bucket| bucket.objects.get_mut(key))
        {
            Some(object) => object,
            None => {
                let err = state
                    .get_object(bucket_name, key, None)
                    .err()
                    .unwrap_or_else(|| no_such_bucket(bucket_name));
                return s3_error_response(&err);
            }
        };
        object.tags.clear();
        wire::serialize_delete_object_tagging_response()
    }

    // -----------------------------------------------------------------------
    // Accelerate Configuration
    // -----------------------------------------------------------------------

    async fn handle_get_bucket_accelerate_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        if !state.buckets.contains_key(bucket_name) {
            return s3_error_response(&no_such_bucket(bucket_name));
        }
        wire::serialize_get_bucket_accelerate_configuration_response(
            &wire::GetBucketAccelerateConfigurationOutput::default(),
        )
    }

    async fn handle_put_bucket_accelerate_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        _body: Bytes,
    ) -> MockResponse {
        let state = state.read().await;
        if !state.buckets.contains_key(bucket_name) {
            return s3_error_response(&no_such_bucket(bucket_name));
        }
        MockResponse {
            status: 200,
            headers: http::HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    // -----------------------------------------------------------------------
    // Request Payment
    // -----------------------------------------------------------------------

    async fn handle_get_bucket_request_payment(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let bs = match state.buckets.get(bucket_name) {
            Some(bs) => bs,
            None => return s3_error_response(&no_such_bucket(bucket_name)),
        };
        let output = wire::GetBucketRequestPaymentOutput {
            payer: Some(bs.request_payment_payer.clone()),
        };
        wire::serialize_get_bucket_request_payment_response(&output)
    }

    async fn handle_put_bucket_request_payment(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        _body: Bytes,
    ) -> MockResponse {
        let state = state.read().await;
        if !state.buckets.contains_key(bucket_name) {
            return s3_error_response(&no_such_bucket(bucket_name));
        }
        MockResponse {
            status: 200,
            headers: http::HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    // -----------------------------------------------------------------------
    // CORS
    // -----------------------------------------------------------------------

    async fn handle_get_bucket_cors(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let bs = match state.buckets.get(bucket_name) {
            Some(bs) => bs,
            None => return s3_error_response(&no_such_bucket(bucket_name)),
        };
        match &bs.cors_configuration {
            Some(xml) => MockResponse::xml(200, xml.clone()),
            None => s3_error_response(&S3Error::NoSuchCORSConfiguration {
                resource: format!("/{bucket_name}?cors"),
            }),
        }
    }

    async fn handle_put_bucket_cors(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        body: Bytes,
    ) -> MockResponse {
        let mut state = state.write().await;
        let bs = match state.buckets.get_mut(bucket_name) {
            Some(bs) => bs,
            None => return s3_error_response(&no_such_bucket(bucket_name)),
        };
        bs.cors_configuration = Some(String::from_utf8_lossy(&body).to_string());
        MockResponse {
            status: 200,
            headers: http::HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    async fn handle_delete_bucket_cors(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        let bs = match state.buckets.get_mut(bucket_name) {
            Some(bs) => bs,
            None => return s3_error_response(&no_such_bucket(bucket_name)),
        };
        bs.cors_configuration = None;
        MockResponse {
            status: 204,
            headers: http::HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    // -----------------------------------------------------------------------
    // Encryption
    // -----------------------------------------------------------------------

    async fn handle_get_bucket_encryption(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let bs = match state.buckets.get(bucket_name) {
            Some(bs) => bs,
            None => return s3_error_response(&no_such_bucket(bucket_name)),
        };
        match &bs.encryption_configuration {
            Some(xml) => MockResponse::xml(200, xml.clone()),
            None => {
                // AWS returns a default SSE-S3 encryption since 2023
                let default_config = wire::ServerSideEncryptionConfiguration {
                    rules: vec![wire::ServerSideEncryptionRule {
                        apply_server_side_encryption_by_default: Some(
                            wire::ServerSideEncryptionByDefault {
                                s_s_e_algorithm: "AES256".to_string(),
                                ..Default::default()
                            },
                        ),
                        bucket_key_enabled: Some(false),
                        ..Default::default()
                    }],
                };
                wire::serialize_get_bucket_encryption_response(&default_config)
            }
        }
    }

    async fn handle_put_bucket_encryption(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        body: Bytes,
    ) -> MockResponse {
        let mut state = state.write().await;
        let bs = match state.buckets.get_mut(bucket_name) {
            Some(bs) => bs,
            None => return s3_error_response(&no_such_bucket(bucket_name)),
        };
        bs.encryption_configuration = Some(String::from_utf8_lossy(&body).to_string());
        MockResponse {
            status: 200,
            headers: http::HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    async fn handle_delete_bucket_encryption(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        let bs = match state.buckets.get_mut(bucket_name) {
            Some(bs) => bs,
            None => return s3_error_response(&no_such_bucket(bucket_name)),
        };
        bs.encryption_configuration = None;
        MockResponse {
            status: 204,
            headers: http::HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    // -----------------------------------------------------------------------
    // Object Lock
    // -----------------------------------------------------------------------

    async fn handle_get_bucket_object_lock_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let Some(bucket) = state.buckets.get(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        match &bucket.object_lock_configuration {
            Some(xml) => MockResponse::xml(200, xml.clone()),
            None => s3_error_response(&S3Error::ObjectLockConfigurationNotFoundError {
                resource: format!("/{bucket_name}?object-lock"),
            }),
        }
    }

    async fn handle_put_object_lock_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        body: Bytes,
    ) -> MockResponse {
        let mut state = state.write().await;
        let Some(bucket) = state.buckets.get_mut(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        bucket.object_lock_configuration = Some(String::from_utf8_lossy(&body).to_string());
        wire::serialize_put_object_lock_configuration_response()
    }

    // -----------------------------------------------------------------------
    // Logging
    // -----------------------------------------------------------------------

    async fn handle_get_bucket_logging(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        if !state.buckets.contains_key(bucket_name) {
            return s3_error_response(&no_such_bucket(bucket_name));
        }
        wire::serialize_get_bucket_logging_response(&wire::GetBucketLoggingOutput::default())
    }

    async fn handle_put_bucket_logging(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        _body: Bytes,
    ) -> MockResponse {
        let state = state.read().await;
        if !state.buckets.contains_key(bucket_name) {
            return s3_error_response(&no_such_bucket(bucket_name));
        }
        MockResponse {
            status: 200,
            headers: http::HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    // -----------------------------------------------------------------------
    // Replication
    // -----------------------------------------------------------------------

    async fn handle_get_bucket_replication(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let Some(bucket) = state.buckets.get(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        match &bucket.replication_configuration {
            Some(xml) => MockResponse::xml(200, xml.clone()),
            None => s3_error_response(&S3Error::ReplicationConfigurationNotFoundError {
                resource: format!("/{bucket_name}?replication"),
            }),
        }
    }

    async fn handle_put_bucket_replication(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        body: Bytes,
    ) -> MockResponse {
        let mut state = state.write().await;
        let Some(bucket) = state.buckets.get_mut(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        bucket.replication_configuration = Some(String::from_utf8_lossy(&body).to_string());
        wire::serialize_put_bucket_replication_response()
    }

    async fn handle_delete_bucket_replication(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        let Some(bucket) = state.buckets.get_mut(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        bucket.replication_configuration = None;
        wire::serialize_delete_bucket_replication_response()
    }

    // -----------------------------------------------------------------------
    // Website
    // -----------------------------------------------------------------------

    async fn handle_get_bucket_website(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let bs = match state.buckets.get(bucket_name) {
            Some(bs) => bs,
            None => return s3_error_response(&no_such_bucket(bucket_name)),
        };
        match &bs.website_configuration {
            Some(xml) => MockResponse::xml(200, xml.clone()),
            None => s3_error_response(&S3Error::NoSuchWebsiteConfiguration {
                resource: format!("/{bucket_name}?website"),
            }),
        }
    }

    async fn handle_put_bucket_website(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        body: Bytes,
    ) -> MockResponse {
        let mut state = state.write().await;
        let bs = match state.buckets.get_mut(bucket_name) {
            Some(bs) => bs,
            None => return s3_error_response(&no_such_bucket(bucket_name)),
        };
        bs.website_configuration = Some(String::from_utf8_lossy(&body).to_string());
        MockResponse {
            status: 200,
            headers: http::HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    async fn handle_delete_bucket_website(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        let bs = match state.buckets.get_mut(bucket_name) {
            Some(bs) => bs,
            None => return s3_error_response(&no_such_bucket(bucket_name)),
        };
        bs.website_configuration = None;
        MockResponse {
            status: 204,
            headers: http::HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    // -----------------------------------------------------------------------
    // Policy Status
    // -----------------------------------------------------------------------

    // STUB[no-engine]: Computing real public-access policy status requires evaluating the
    //   bucket policy document for public-facing statements; always returns IsPublic=false.
    async fn handle_get_bucket_policy_status(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        if !state.buckets.contains_key(bucket_name) {
            return s3_error_response(&no_such_bucket(bucket_name));
        }
        wire::serialize_get_bucket_policy_status_response(&wire::PolicyStatus {
            is_public: Some(false),
        })
    }

    // -----------------------------------------------------------------------
    // Ownership Controls
    // -----------------------------------------------------------------------

    async fn handle_get_bucket_ownership_controls(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let bs = match state.buckets.get(bucket_name) {
            Some(bs) => bs,
            None => return s3_error_response(&no_such_bucket(bucket_name)),
        };
        let object_ownership = bs
            .ownership_controls
            .as_deref()
            .unwrap_or("BucketOwnerEnforced");
        wire::serialize_get_bucket_ownership_controls_response(&wire::OwnershipControls {
            rules: vec![wire::OwnershipControlsRule {
                object_ownership: object_ownership.to_string(),
            }],
        })
    }

    async fn handle_put_bucket_ownership_controls(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        body: Bytes,
    ) -> MockResponse {
        let mut state = state.write().await;
        let bs = match state.buckets.get_mut(bucket_name) {
            Some(bs) => bs,
            None => return s3_error_response(&no_such_bucket(bucket_name)),
        };
        let body_str = String::from_utf8_lossy(&body);
        // Extract ObjectOwnership value
        if let Some(start) = body_str.find("<ObjectOwnership>") {
            let after = &body_str[start + 17..];
            if let Some(end) = after.find("</ObjectOwnership>") {
                bs.ownership_controls = Some(after[..end].to_string());
            }
        }
        MockResponse {
            status: 200,
            headers: http::HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    async fn handle_delete_bucket_ownership_controls(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        let bs = match state.buckets.get_mut(bucket_name) {
            Some(bs) => bs,
            None => return s3_error_response(&no_such_bucket(bucket_name)),
        };
        bs.ownership_controls = None;
        MockResponse {
            status: 204,
            headers: http::HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    // -----------------------------------------------------------------------
    // Public Access Block
    // -----------------------------------------------------------------------

    async fn handle_get_public_access_block(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let bs = match state.buckets.get(bucket_name) {
            Some(bs) => bs,
            None => return s3_error_response(&no_such_bucket(bucket_name)),
        };
        match &bs.public_access_block {
            Some(cfg) => wire::serialize_get_public_access_block_response(
                &wire::PublicAccessBlockConfiguration {
                    block_public_acls: Some(cfg.block_public_acls),
                    ignore_public_acls: Some(cfg.ignore_public_acls),
                    block_public_policy: Some(cfg.block_public_policy),
                    restrict_public_buckets: Some(cfg.restrict_public_buckets),
                },
            ),
            None => s3_error_response(&S3Error::NoSuchPublicAccessBlockConfiguration {
                resource: format!("/{bucket_name}?publicAccessBlock"),
            }),
        }
    }

    async fn handle_put_public_access_block(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        body: Bytes,
    ) -> MockResponse {
        use crate::state::PublicAccessBlockConfig;
        let mut state = state.write().await;
        let bs = match state.buckets.get_mut(bucket_name) {
            Some(bs) => bs,
            None => return s3_error_response(&no_such_bucket(bucket_name)),
        };
        let body_str = String::from_utf8_lossy(&body);
        let extract_bool = |tag: &str| -> bool {
            body_str
                .find(&format!("<{tag}>"))
                .and_then(|start| {
                    let after = &body_str[start + tag.len() + 2..];
                    after.find(&format!("</{tag}>")).map(|end| &after[..end])
                })
                .map(|v| v == "true" || v == "TRUE")
                .unwrap_or(false)
        };
        bs.public_access_block = Some(PublicAccessBlockConfig {
            block_public_acls: extract_bool("BlockPublicAcls"),
            ignore_public_acls: extract_bool("IgnorePublicAcls"),
            block_public_policy: extract_bool("BlockPublicPolicy"),
            restrict_public_buckets: extract_bool("RestrictPublicBuckets"),
        });
        MockResponse {
            status: 200,
            headers: http::HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    async fn handle_delete_public_access_block(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        let bs = match state.buckets.get_mut(bucket_name) {
            Some(bs) => bs,
            None => return s3_error_response(&no_such_bucket(bucket_name)),
        };
        bs.public_access_block = None;
        MockResponse {
            status: 204,
            headers: http::HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    // -----------------------------------------------------------------------
    // Lifecycle (DELETE)
    // -----------------------------------------------------------------------

    async fn handle_delete_bucket_lifecycle(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        let bs = match state.buckets.get_mut(bucket_name) {
            Some(bs) => bs,
            None => return s3_error_response(&no_such_bucket(bucket_name)),
        };
        bs.lifecycle_configuration = None;
        MockResponse {
            status: 204,
            headers: http::HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    // --- Analytics Configuration ---

    async fn handle_put_bucket_analytics_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        query: &std::collections::HashMap<String, String>,
        request: &MockRequest,
    ) -> MockResponse {
        let input = match deserialize_s3_request(
            wire::deserialize_put_bucket_analytics_configuration_request(
                request,
                &[("Bucket", bucket_name)],
                query,
            ),
            &request.uri,
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        if input.id.is_empty() {
            return s3_error_response(&S3Error::MissingAnalyticsConfigurationId {
                resource: format!("/{bucket_name}?analytics"),
            });
        }
        let mut state = state.write().await;
        let Some(bucket) = state.buckets.get_mut(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        bucket.analytics_configurations.insert(
            input.id.clone(),
            quick_xml::se::to_string(&input.analytics_configuration).unwrap_or_default(),
        );
        wire::serialize_put_bucket_analytics_configuration_response()
    }

    async fn handle_get_bucket_analytics_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        query: &std::collections::HashMap<String, String>,
    ) -> MockResponse {
        let id = match query.get("id") {
            Some(value) => value,
            None => {
                return s3_error_response(&S3Error::MissingAnalyticsConfigurationId {
                    resource: format!("/{bucket_name}?analytics"),
                });
            }
        };
        let state = state.read().await;
        let Some(bucket) = state.buckets.get(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        match bucket.analytics_configurations.get(id) {
            Some(xml) => match quick_xml::de::from_str::<wire::AnalyticsConfiguration>(xml) {
                Ok(config) => wire::serialize_get_bucket_analytics_configuration_response(&config),
                Err(_) => MockResponse::xml(200, xml.clone()),
            },
            None => s3_error_response(&S3Error::NoSuchAnalyticsConfiguration {
                resource: format!("/{bucket_name}?analytics&id={id}"),
            }),
        }
    }

    async fn handle_delete_bucket_analytics_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        query: &std::collections::HashMap<String, String>,
        request: &MockRequest,
    ) -> MockResponse {
        let input = match deserialize_s3_request(
            wire::deserialize_delete_bucket_analytics_configuration_request(
                request,
                &[("Bucket", bucket_name)],
                query,
            ),
            &request.uri,
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let mut state = state.write().await;
        let Some(bucket) = state.buckets.get_mut(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        bucket.analytics_configurations.remove(&input.id);
        wire::serialize_delete_bucket_analytics_configuration_response()
    }

    async fn handle_list_bucket_analytics_configurations(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let Some(bucket) = state.buckets.get(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        let configurations = bucket
            .analytics_configurations
            .values()
            .filter_map(|xml| quick_xml::de::from_str::<wire::AnalyticsConfiguration>(xml).ok())
            .collect::<Vec<_>>();
        let result = wire::ListBucketAnalyticsConfigurationsOutput {
            analytics_configuration_list: (!configurations.is_empty()).then_some(configurations),
            continuation_token: None,
            is_truncated: Some(false),
            next_continuation_token: None,
        };
        wire::serialize_list_bucket_analytics_configurations_response(&result)
    }

    // --- Intelligent Tiering Configuration ---

    async fn handle_put_bucket_intelligent_tiering_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        query: &std::collections::HashMap<String, String>,
        request: &MockRequest,
    ) -> MockResponse {
        let input = match deserialize_s3_request(
            wire::deserialize_put_bucket_intelligent_tiering_configuration_request(
                request,
                &[("Bucket", bucket_name)],
                query,
            ),
            &request.uri,
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        if input.id.is_empty() {
            return s3_error_response(&S3Error::MissingIntelligentTieringConfigurationId {
                resource: format!("/{bucket_name}?intelligent-tiering"),
            });
        }
        let mut state = state.write().await;
        let Some(bucket) = state.buckets.get_mut(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        bucket.intelligent_tiering_configurations.insert(
            input.id.clone(),
            quick_xml::se::to_string(&input.intelligent_tiering_configuration).unwrap_or_default(),
        );
        wire::serialize_put_bucket_intelligent_tiering_configuration_response()
    }

    async fn handle_get_bucket_intelligent_tiering_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        query: &std::collections::HashMap<String, String>,
    ) -> MockResponse {
        let id = match query.get("id") {
            Some(value) => value,
            None => {
                return s3_error_response(&S3Error::MissingIntelligentTieringConfigurationId {
                    resource: format!("/{bucket_name}?intelligent-tiering"),
                });
            }
        };
        let state = state.read().await;
        let Some(bucket) = state.buckets.get(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        match bucket.intelligent_tiering_configurations.get(id) {
            Some(xml) => {
                match quick_xml::de::from_str::<wire::IntelligentTieringConfiguration>(xml) {
                    Ok(config) => {
                        wire::serialize_get_bucket_intelligent_tiering_configuration_response(
                            &config,
                        )
                    }
                    Err(_) => MockResponse::xml(200, xml.clone()),
                }
            }
            None => s3_error_response(&S3Error::NoSuchIntelligentTieringConfiguration {
                resource: format!("/{bucket_name}?intelligent-tiering&id={id}"),
            }),
        }
    }

    async fn handle_delete_bucket_intelligent_tiering_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        query: &std::collections::HashMap<String, String>,
        request: &MockRequest,
    ) -> MockResponse {
        let input = match deserialize_s3_request(
            wire::deserialize_delete_bucket_intelligent_tiering_configuration_request(
                request,
                &[("Bucket", bucket_name)],
                query,
            ),
            &request.uri,
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let mut state = state.write().await;
        let Some(bucket) = state.buckets.get_mut(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        bucket.intelligent_tiering_configurations.remove(&input.id);
        wire::serialize_delete_bucket_intelligent_tiering_configuration_response()
    }

    async fn handle_list_bucket_intelligent_tiering_configurations(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let Some(bucket) = state.buckets.get(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        let configurations = bucket
            .intelligent_tiering_configurations
            .values()
            .filter_map(|xml| {
                quick_xml::de::from_str::<wire::IntelligentTieringConfiguration>(xml).ok()
            })
            .collect::<Vec<_>>();
        let result = wire::ListBucketIntelligentTieringConfigurationsOutput {
            intelligent_tiering_configuration_list: (!configurations.is_empty())
                .then_some(configurations),
            continuation_token: None,
            is_truncated: Some(false),
            next_continuation_token: None,
        };
        wire::serialize_list_bucket_intelligent_tiering_configurations_response(&result)
    }

    // --- Metrics Configuration ---

    async fn handle_put_bucket_metrics_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        query: &std::collections::HashMap<String, String>,
        request: &MockRequest,
    ) -> MockResponse {
        let input = match deserialize_s3_request(
            wire::deserialize_put_bucket_metrics_configuration_request(
                request,
                &[("Bucket", bucket_name)],
                query,
            ),
            &request.uri,
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        if input.id.is_empty() {
            return s3_error_response(&S3Error::MissingMetricsConfigurationId {
                resource: format!("/{bucket_name}?metrics"),
            });
        }
        let mut state = state.write().await;
        let Some(bucket) = state.buckets.get_mut(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        bucket.metrics_configurations.insert(
            input.id.clone(),
            quick_xml::se::to_string(&input.metrics_configuration).unwrap_or_default(),
        );
        wire::serialize_put_bucket_metrics_configuration_response()
    }

    async fn handle_get_bucket_metrics_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        query: &std::collections::HashMap<String, String>,
    ) -> MockResponse {
        let id = match query.get("id") {
            Some(value) => value,
            None => {
                return s3_error_response(&S3Error::MissingMetricsConfigurationId {
                    resource: format!("/{bucket_name}?metrics"),
                });
            }
        };
        let state = state.read().await;
        let Some(bucket) = state.buckets.get(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        match bucket.metrics_configurations.get(id) {
            Some(xml) => match quick_xml::de::from_str::<wire::MetricsConfiguration>(xml) {
                Ok(config) => wire::serialize_get_bucket_metrics_configuration_response(&config),
                Err(_) => MockResponse::xml(200, xml.clone()),
            },
            None => s3_error_response(&S3Error::NoSuchMetricsConfiguration {
                resource: format!("/{bucket_name}?metrics&id={id}"),
            }),
        }
    }

    async fn handle_delete_bucket_metrics_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        query: &std::collections::HashMap<String, String>,
        request: &MockRequest,
    ) -> MockResponse {
        let input = match deserialize_s3_request(
            wire::deserialize_delete_bucket_metrics_configuration_request(
                request,
                &[("Bucket", bucket_name)],
                query,
            ),
            &request.uri,
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let mut state = state.write().await;
        let Some(bucket) = state.buckets.get_mut(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        bucket.metrics_configurations.remove(&input.id);
        wire::serialize_delete_bucket_metrics_configuration_response()
    }

    async fn handle_list_bucket_metrics_configurations(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let Some(bucket) = state.buckets.get(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        let configurations = bucket
            .metrics_configurations
            .values()
            .filter_map(|xml| quick_xml::de::from_str::<wire::MetricsConfiguration>(xml).ok())
            .collect::<Vec<_>>();
        let result = wire::ListBucketMetricsConfigurationsOutput {
            metrics_configuration_list: (!configurations.is_empty()).then_some(configurations),
            continuation_token: None,
            is_truncated: Some(false),
            next_continuation_token: None,
        };
        wire::serialize_list_bucket_metrics_configurations_response(&result)
    }

    // --- ABAC ---

    async fn handle_put_bucket_abac(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        request: &MockRequest,
    ) -> MockResponse {
        let xml = String::from_utf8_lossy(&request.body).to_string();
        let mut state = state.write().await;
        let Some(bucket) = state.buckets.get_mut(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        bucket.abac_status = Some(xml);
        wire::serialize_put_bucket_abac_response()
    }

    async fn handle_get_bucket_abac(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let Some(bucket) = state.buckets.get(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        match &bucket.abac_status {
            Some(xml) => match quick_xml::de::from_str::<wire::AbacStatus>(xml) {
                Ok(status) => wire::serialize_get_bucket_abac_response(&status),
                Err(_) => MockResponse::xml(200, xml.clone()),
            },
            None => {
                let result = wire::AbacStatus { status: None };
                wire::serialize_get_bucket_abac_response(&result)
            }
        }
    }

    // --- Session ---

    // STUB[no-auth]: The mock has no real session management layer; always returns
    //   hardcoded mock credentials valid until 2099.
    async fn handle_create_session(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        if !state.buckets.contains_key(bucket_name) {
            return s3_error_response(&no_such_bucket(bucket_name));
        }
        let result = wire::CreateSessionOutput {
            credentials: Some(wire::SessionCredentials {
                access_key_id: Some("AKIAIOSFODNN7EXAMPLE".to_string()),
                expiration: Some("2099-01-01T00:00:00Z".to_string()),
                secret_access_key: Some("wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY".to_string()),
                session_token: Some("mock-session-token".to_string()),
            }),
            ..Default::default()
        };
        wire::serialize_create_session_response(&result)
    }

    // --- Directory Buckets ---

    async fn handle_list_directory_buckets(&self) -> MockResponse {
        let result = wire::ListDirectoryBucketsOutput {
            buckets: None,
            continuation_token: None,
        };
        wire::serialize_list_directory_buckets_response(&result)
    }

    // --- Inventory deletion ---

    async fn handle_delete_bucket_inventory_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        query: &std::collections::HashMap<String, String>,
        request: &MockRequest,
    ) -> MockResponse {
        let input = match deserialize_s3_request(
            wire::deserialize_delete_bucket_inventory_configuration_request(
                request,
                &[("Bucket", bucket_name)],
                query,
            ),
            &request.uri,
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let mut state = state.write().await;
        let Some(bucket) = state.buckets.get_mut(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        bucket.inventory_configurations.remove(&input.id);
        wire::serialize_delete_bucket_inventory_configuration_response()
    }

    // --- Metadata Table Configuration ---

    async fn handle_create_bucket_metadata_table_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        request: &MockRequest,
    ) -> MockResponse {
        let xml = String::from_utf8_lossy(&request.body).to_string();
        let mut state = state.write().await;
        let Some(bucket) = state.buckets.get_mut(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        bucket.metadata_table_configuration = Some(xml);
        wire::serialize_create_bucket_metadata_table_configuration_response()
    }

    async fn handle_get_bucket_metadata_table_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let Some(bucket) = state.buckets.get(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        match &bucket.metadata_table_configuration {
            Some(xml) => match quick_xml::de::from_str::<
                wire::GetBucketMetadataTableConfigurationResult,
            >(xml)
            {
                Ok(result) => {
                    wire::serialize_get_bucket_metadata_table_configuration_response(&result)
                }
                Err(_) => MockResponse::xml(200, xml.clone()),
            },
            None => s3_error_response(&S3Error::NoSuchMetadataTableConfiguration {
                resource: format!("/{bucket_name}?metadataTable"),
            }),
        }
    }

    async fn handle_delete_bucket_metadata_table_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        let Some(bucket) = state.buckets.get_mut(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        bucket.metadata_table_configuration = None;
        wire::serialize_delete_bucket_metadata_table_configuration_response()
    }

    // --- Metadata Configuration ---

    async fn handle_create_bucket_metadata_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        request: &MockRequest,
    ) -> MockResponse {
        let xml = String::from_utf8_lossy(&request.body).to_string();
        let mut state = state.write().await;
        let Some(bucket) = state.buckets.get_mut(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        bucket.metadata_configuration = Some(xml);
        wire::serialize_create_bucket_metadata_configuration_response()
    }

    async fn handle_get_bucket_metadata_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let Some(bucket) = state.buckets.get(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        match &bucket.metadata_configuration {
            Some(xml) => {
                match quick_xml::de::from_str::<wire::GetBucketMetadataConfigurationResult>(xml) {
                    Ok(result) => {
                        wire::serialize_get_bucket_metadata_configuration_response(&result)
                    }
                    Err(_) => MockResponse::xml(200, xml.clone()),
                }
            }
            None => s3_error_response(&S3Error::NoSuchMetadataConfiguration {
                resource: format!("/{bucket_name}?metadataConfiguration"),
            }),
        }
    }

    async fn handle_delete_bucket_metadata_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        let Some(bucket) = state.buckets.get_mut(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        bucket.metadata_configuration = None;
        wire::serialize_delete_bucket_metadata_configuration_response()
    }

    async fn handle_update_bucket_metadata_inventory_table_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        _request: &MockRequest,
    ) -> MockResponse {
        // Validate bucket exists
        let state = state.read().await;
        if !state.buckets.contains_key(bucket_name) {
            return s3_error_response(&no_such_bucket(bucket_name));
        }
        wire::serialize_update_bucket_metadata_inventory_table_configuration_response()
    }

    async fn handle_update_bucket_metadata_journal_table_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        _request: &MockRequest,
    ) -> MockResponse {
        // Validate bucket exists
        let state = state.read().await;
        if !state.buckets.contains_key(bucket_name) {
            return s3_error_response(&no_such_bucket(bucket_name));
        }
        wire::serialize_update_bucket_metadata_journal_table_configuration_response()
    }

    // --- Object torrent ---

    // STUB[no-engine]: Generating a BitTorrent descriptor requires a real torrent-creation
    //   engine; the mock validates the object exists and returns an empty body.
    async fn handle_get_object_torrent(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        key: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_object(bucket_name, key, None) {
            Ok(_) => wire::serialize_get_object_torrent_response(),
            Err(e) => s3_error_response(&e),
        }
    }

    // --- Rename Object ---

    async fn handle_rename_object(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        key: &str,
        request: &MockRequest,
    ) -> MockResponse {
        // x-amz-rename-source header contains the source key (as /bucket/key or just key)
        let rename_source = match request
            .headers
            .get("x-amz-rename-source")
            .and_then(|v| v.to_str().ok())
        {
            Some(k) => k.to_string(),
            None => {
                return s3_error_response(&S3Error::MissingRenameSourceHeader {
                    resource: format!("/{bucket_name}/{key}"),
                });
            }
        };
        // The source may be "/bucket/key" or just "key"
        let source_key = if rename_source.contains('/') {
            rename_source
                .splitn(3, '/')
                .nth(2)
                .unwrap_or(&rename_source)
                .to_string()
        } else {
            rename_source
        };
        let mut state = state.write().await;
        let Some(bucket) = state.buckets.get_mut(bucket_name) else {
            return s3_error_response(&no_such_bucket(bucket_name));
        };
        let obj = match bucket.objects.remove(&source_key) {
            Some(obj) => obj,
            None => {
                return s3_error_response(&S3Error::NoSuchKey {
                    resource: format!("/{bucket_name}/{source_key}"),
                });
            }
        };
        let mut renamed = obj;
        renamed.key = key.to_string();
        bucket.objects.insert(key.to_string(), renamed);
        wire::serialize_rename_object_response()
    }

    // --- Update Object Encryption ---

    async fn handle_update_object_encryption(
        &self,
        state: &Arc<tokio::sync::RwLock<S3State>>,
        bucket_name: &str,
        key: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_object(bucket_name, key, None) {
            Ok(_) => wire::serialize_update_object_encryption_response(),
            Err(e) => s3_error_response(&e),
        }
    }

    // --- S3 Control (account-level) dispatch ---

    // FIX(terraform-e2e): terraform AWS provider calls S3 Control ListTagsForResource during
    //   read of aws_s3_bucket resources. S3 Control signs with service name "s3" so requests
    //   are routed to S3Service. This dispatcher handles /v20180820/ paths that correspond to
    //   the S3 Control REST API.
    async fn dispatch_s3control(&self, request: MockRequest) -> MockResponse {
        let (_, path) = match extract_host_and_path(&request.uri) {
            Some(v) => v,
            None => {
                return MockResponse::json(400, r#"{"message":"Invalid URI"}"#.to_string());
            }
        };
        let path_no_qs = path.split('?').next().unwrap_or(path);
        let method = request.method.as_str();

        // GET /v20180820/tags/{arn} - ListTagsForResource
        if method == "GET" && path_no_qs.starts_with("/v20180820/tags/") {
            return MockResponse::json(200, r#"{"Tags":[]}"#.to_string());
        }
        // PUT /v20180820/tags/{arn} - PutTagsForResource
        if method == "PUT" && path_no_qs.starts_with("/v20180820/tags/") {
            return MockResponse {
                status: 204,
                headers: Default::default(),
                body: Default::default(),
            };
        }
        // DELETE /v20180820/tags/{arn} - DeleteTagsForResource
        if method == "DELETE" && path_no_qs.starts_with("/v20180820/tags/") {
            return MockResponse {
                status: 204,
                headers: Default::default(),
                body: Default::default(),
            };
        }
        // Catch-all: accept all other S3 Control paths with 200 empty
        MockResponse::json(200, r#"{}"#.to_string())
    }
}

// --- URL parsing ---

/// Parse an S3 request URI into bucket, key, and region.
///
/// Handles both virtual-hosted-style and path-style URLs:
/// - Virtual-hosted: `https://bucket.s3.us-east-1.amazonaws.com/key`
/// - Path-style: `https://s3.us-east-1.amazonaws.com/bucket/key`
fn parse_s3_target(uri: &str) -> Option<S3Target> {
    let (host, path_and_query) = extract_host_and_path(uri)?;

    // Remove query string from path
    let path = path_and_query.split('?').next().unwrap_or(path_and_query);

    // Check if this is virtual-hosted style
    // Virtual-hosted: bucket.s3.region.amazonaws.com or bucket.s3.amazonaws.com
    if let Some(bucket_name) = extract_vhost_bucket(host) {
        let region = extract_s3_region(host);
        let key = if path.len() > 1 {
            Some(percent_decode(&path[1..])) // strip leading /
        } else {
            None
        };
        return Some(S3Target {
            bucket: Some(bucket_name),
            key,
            region,
        });
    }

    // Path-style: s3.region.amazonaws.com/bucket/key or localhost/bucket/key
    let region = extract_s3_region(host);
    let trimmed = path.strip_prefix('/').unwrap_or(path);
    if trimmed.is_empty() {
        // No bucket specified - service-level request (e.g., ListBuckets)
        return Some(S3Target {
            bucket: None,
            key: None,
            region,
        });
    }

    let (bucket, key) = if let Some(pos) = trimmed.find('/') {
        let bucket = &trimmed[..pos];
        let key_part = &trimmed[pos + 1..];
        if key_part.is_empty() {
            (bucket.to_string(), None)
        } else {
            (bucket.to_string(), Some(percent_decode(key_part)))
        }
    } else {
        (trimmed.to_string(), None)
    };

    Some(S3Target {
        bucket: Some(bucket),
        key,
        region,
    })
}

/// Extract bucket name from virtual-hosted-style host.
/// e.g., "my-bucket.s3.us-east-1.amazonaws.com" -> Some("my-bucket")
fn extract_vhost_bucket(host: &str) -> Option<String> {
    // Pattern: {bucket}.s3{...}.amazonaws.com
    // We check if the host has a ".s3" segment that's NOT at the start
    let s3_pos = host.find(".s3")?;
    if s3_pos == 0 {
        return None; // Path-style: s3.region.amazonaws.com
    }
    let bucket = &host[..s3_pos];
    if bucket.is_empty() {
        return None;
    }
    Some(bucket.to_string())
}

/// Extract region from S3 host.
fn extract_s3_region(host: &str) -> String {
    // Try to find the pattern: s3.REGION.amazonaws.com or s3-REGION.amazonaws.com
    let parts: Vec<&str> = host.split('.').collect();
    for (i, part) in parts.iter().enumerate() {
        if part.starts_with("s3") && i + 2 < parts.len() {
            if parts[i + 1] != "amazonaws" {
                return parts[i + 1].to_string();
            }
        }
        // Handle s3-REGION.amazonaws.com
        if let Some(region) = part.strip_prefix("s3-") {
            return region.to_string();
        }
    }
    "us-east-1".to_string()
}

fn extract_host_and_path(uri: &str) -> Option<(&str, &str)> {
    let after_scheme = uri
        .strip_prefix("https://")
        .or_else(|| uri.strip_prefix("http://"))?;
    let slash_pos = after_scheme.find('/').unwrap_or(after_scheme.len());
    let host = &after_scheme[..slash_pos];
    // Strip port
    let host = host.split(':').next().unwrap_or(host);
    let path = if slash_pos < after_scheme.len() {
        &after_scheme[slash_pos..]
    } else {
        "/"
    };
    Some((host, path))
}

fn parse_query_from_uri(uri: &str) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    if let Some(qs) = uri.split('?').nth(1) {
        for pair in qs.split('&') {
            if let Some((key, value)) = pair.split_once('=') {
                map.insert(percent_decode(key), percent_decode(value));
            } else if !pair.is_empty() {
                map.insert(percent_decode(pair), String::new());
            }
        }
    }
    map
}

fn percent_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut bytes = s.bytes();
    while let Some(b) = bytes.next() {
        match b {
            b'%' => {
                let hi = bytes.next().and_then(hex_val);
                let lo = bytes.next().and_then(hex_val);
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    result.push((hi << 4 | lo) as char);
                }
            }
            b'+' => result.push(' '),
            _ => result.push(b as char),
        }
    }
    result
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn to_list_object(object: &crate::types::Object) -> wire::Object {
    wire::Object {
        checksum_algorithm: None,
        checksum_type: None,
        e_tag: Some(object.etag.clone()),
        key: Some(object.key.clone()),
        last_modified: Some(
            object
                .last_modified
                .format("%Y-%m-%dT%H:%M:%S.000Z")
                .to_string(),
        ),
        owner: None,
        restore_status: None,
        size: Some(object.size() as i64),
        storage_class: Some(object.storage_class.clone()),
    }
}

fn default_owner() -> wire::Owner {
    wire::Owner {
        display_name: Some("winterbaume".to_string()),
        i_d: Some(DEFAULT_ACCOUNT_ID.to_string()),
    }
}

fn default_initiator() -> wire::Initiator {
    wire::Initiator {
        display_name: Some("winterbaume".to_string()),
        i_d: Some(DEFAULT_ACCOUNT_ID.to_string()),
    }
}

fn owner_full_control_grant() -> wire::Grant {
    wire::Grant {
        grantee: Some(wire::Grantee {
            display_name: Some("winterbaume".to_string()),
            email_address: None,
            i_d: Some(DEFAULT_ACCOUNT_ID.to_string()),
            r#type: "CanonicalUser".to_string(),
            u_r_i: None,
        }),
        permission: Some("FULL_CONTROL".to_string()),
    }
}

fn public_read_grant() -> wire::Grant {
    wire::Grant {
        grantee: Some(wire::Grantee {
            display_name: None,
            email_address: None,
            i_d: None,
            r#type: "Group".to_string(),
            u_r_i: Some("http://acs.amazonaws.com/groups/global/AllUsers".to_string()),
        }),
        permission: Some("READ".to_string()),
    }
}

fn parse_copy_source_header(
    request: &MockRequest,
    resource: &str,
) -> Result<(String, String), MockResponse> {
    let copy_source = match request
        .headers
        .get("x-amz-copy-source")
        .and_then(|value| value.to_str().ok())
    {
        Some(value) => percent_decode(value).trim_start_matches('/').to_string(),
        None => {
            return Err(s3_error_response(&S3Error::MissingCopySourceHeader {
                resource: resource.to_string(),
            }));
        }
    };
    match copy_source.split_once('/') {
        Some((bucket, key)) => Ok((bucket.to_string(), key.to_string())),
        None => Err(s3_error_response(&S3Error::InvalidCopySourceHeader {
            resource: resource.to_string(),
        })),
    }
}

fn deserialize_s3_request<T>(result: Result<T, String>, resource: &str) -> Result<T, MockResponse> {
    result.map_err(|message| {
        s3_error_response(&S3Error::MalformedXML {
            message,
            resource: resource.to_string(),
        })
    })
}

fn s3_error_response(err: &S3Error) -> MockResponse {
    let (status, code) = match err {
        S3Error::InvalidBucketName { .. } => (400, "InvalidBucketName"),
        S3Error::BucketAlreadyOwnedByYou { .. } => (409, "BucketAlreadyOwnedByYou"),
        S3Error::BucketNotEmpty { .. } => (409, "BucketNotEmpty"),
        S3Error::NoSuchBucket { .. } => (404, "NoSuchBucket"),
        S3Error::NoSuchKey { .. } => (404, "NoSuchKey"),
        S3Error::NoSuchUpload { .. } => (404, "NoSuchUpload"),
        S3Error::InvalidPartNumber { .. } => (400, "InvalidArgument"),
        S3Error::InvalidPart { .. } => (400, "InvalidPart"),
        S3Error::InvalidRequest { .. } => (400, "InvalidRequest"),
        S3Error::MissingCopySourceHeader { .. } => (400, "InvalidRequest"),
        S3Error::InvalidCopySourceHeader { .. } => (400, "InvalidRequest"),
        S3Error::MethodNotAllowed { .. } => (405, "MethodNotAllowed"),
        S3Error::InternalError { .. } => (500, "InternalError"),
        S3Error::MalformedXML { .. } => (400, "MalformedXML"),
        S3Error::MissingInventoryConfigurationId { .. } => (400, "InvalidArgument"),
        S3Error::NoSuchInventoryConfiguration { .. } => (404, "NoSuchConfiguration"),
        S3Error::NoSuchLifecycleConfiguration { .. } => (404, "NoSuchLifecycleConfiguration"),
        S3Error::NoSuchBucketPolicy { .. } => (404, "NoSuchBucketPolicy"),
        S3Error::NoSuchTagSet { .. } => (404, "NoSuchTagSet"),
        S3Error::NoSuchCORSConfiguration { .. } => (404, "NoSuchCORSConfiguration"),
        S3Error::ObjectLockConfigurationNotFoundError { .. } => {
            (404, "ObjectLockConfigurationNotFoundError")
        }
        S3Error::ReplicationConfigurationNotFoundError { .. } => {
            (404, "ReplicationConfigurationNotFoundError")
        }
        S3Error::NoSuchWebsiteConfiguration { .. } => (404, "NoSuchWebsiteConfiguration"),
        S3Error::NoSuchPublicAccessBlockConfiguration { .. } => {
            (404, "NoSuchPublicAccessBlockConfiguration")
        }
        S3Error::MissingAnalyticsConfigurationId { .. } => (400, "InvalidArgument"),
        S3Error::NoSuchAnalyticsConfiguration { .. } => (404, "NoSuchConfiguration"),
        S3Error::MissingIntelligentTieringConfigurationId { .. } => (400, "InvalidArgument"),
        S3Error::NoSuchIntelligentTieringConfiguration { .. } => (404, "NoSuchConfiguration"),
        S3Error::MissingMetricsConfigurationId { .. } => (400, "InvalidArgument"),
        S3Error::NoSuchMetricsConfiguration { .. } => (404, "NoSuchConfiguration"),
        S3Error::NoSuchMetadataTableConfiguration { .. } => (404, "NoSuchConfiguration"),
        S3Error::NoSuchMetadataConfiguration { .. } => (404, "NoSuchConfiguration"),
        S3Error::MissingRenameSourceHeader { .. } => (400, "InvalidArgument"),
    };
    let body = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<Error>
  <Code>{code}</Code>
  <Message>{message}</Message>
  <Resource>{resource}</Resource>
  <RequestId>{request_id}</RequestId>
</Error>"#,
        code = xml_escape(code),
        message = xml_escape(&err.to_string()),
        resource = xml_escape(err.resource()),
        request_id = uuid::Uuid::new_v4(),
    );
    MockResponse::xml(status, body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_vhost_style() {
        let target =
            parse_s3_target("https://my-bucket.s3.us-east-1.amazonaws.com/my/key").unwrap();
        assert_eq!(target.bucket.as_deref(), Some("my-bucket"));
        assert_eq!(target.key.as_deref(), Some("my/key"));
        assert_eq!(target.region, "us-east-1");
    }

    #[test]
    fn test_parse_vhost_style_no_key() {
        let target = parse_s3_target("https://my-bucket.s3.us-east-1.amazonaws.com/").unwrap();
        assert_eq!(target.bucket.as_deref(), Some("my-bucket"));
        assert!(target.key.is_none());
    }

    #[test]
    fn test_parse_path_style() {
        let target =
            parse_s3_target("https://s3.us-east-1.amazonaws.com/my-bucket/my/key").unwrap();
        assert_eq!(target.bucket.as_deref(), Some("my-bucket"));
        assert_eq!(target.key.as_deref(), Some("my/key"));
        assert_eq!(target.region, "us-east-1");
    }

    #[test]
    fn test_parse_path_style_no_key() {
        let target = parse_s3_target("https://s3.us-east-1.amazonaws.com/my-bucket").unwrap();
        assert_eq!(target.bucket.as_deref(), Some("my-bucket"));
        assert!(target.key.is_none());
    }

    #[test]
    fn test_parse_global_vhost() {
        let target = parse_s3_target("https://my-bucket.s3.amazonaws.com/key").unwrap();
        assert_eq!(target.bucket.as_deref(), Some("my-bucket"));
        assert_eq!(target.key.as_deref(), Some("key"));
        assert_eq!(target.region, "us-east-1");
    }

    #[test]
    fn test_parse_localhost_no_bucket() {
        let target = parse_s3_target("https://localhost:5555/").unwrap();
        assert!(target.bucket.is_none());
        assert!(target.key.is_none());
    }

    #[test]
    fn test_parse_localhost_with_bucket() {
        let target = parse_s3_target("https://localhost:5555/my-bucket").unwrap();
        assert_eq!(target.bucket.as_deref(), Some("my-bucket"));
        assert!(target.key.is_none());
    }

    #[test]
    fn test_parse_localhost_with_bucket_and_key() {
        let target = parse_s3_target("https://localhost:5555/my-bucket/my/key").unwrap();
        assert_eq!(target.bucket.as_deref(), Some("my-bucket"));
        assert_eq!(target.key.as_deref(), Some("my/key"));
    }

    #[test]
    fn test_extract_vhost_bucket() {
        assert_eq!(
            extract_vhost_bucket("my-bucket.s3.us-east-1.amazonaws.com"),
            Some("my-bucket".to_string())
        );
        assert_eq!(extract_vhost_bucket("s3.us-east-1.amazonaws.com"), None);
    }
}

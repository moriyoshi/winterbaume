use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use tokio::io::AsyncReadExt;
use tokio::sync::RwLock;
use winterbaume_core::{
    BackendState, BlobStoreMap, MockRequest, MockResponse, MockService, StateChangeNotifier, Vfs,
    default_account_id,
};

use crate::state::{GlacierError, GlacierState, JobOutput};
use crate::types::{DataRetrievalPolicy, DataRetrievalRule, JobType};
use crate::views::GlacierStateView;
use crate::wire;

pub struct GlacierService {
    pub(crate) state: Arc<BackendState<GlacierState>>,
    pub(crate) blobs: Arc<BlobStoreMap>,
    pub(crate) notifier: StateChangeNotifier<GlacierStateView>,
}

impl GlacierService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            blobs: Arc::new(BlobStoreMap::mem("glacier")),
            notifier: StateChangeNotifier::new(),
        }
    }

    pub fn with_vfs(vfs: Arc<dyn Vfs>) -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            blobs: Arc::new(BlobStoreMap::new(vfs, "glacier")),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for GlacierService {
    fn default() -> Self {
        Self::new()
    }
}

impl GlacierService {
    /// Reconstruct state from the BlobStore metadata alone (no snapshot).
    ///
    /// Walks the `bucket-config` namespace for vault configs and
    /// `version-meta` namespace for archive metadata.  Errors on individual
    /// records are silently skipped for resilience.
    pub(crate) async fn recover_state_from_blobs(
        blobs: &winterbaume_core::BlobStore,
    ) -> Result<GlacierState, winterbaume_core::VfsError> {
        use crate::stored::{StoredArchiveMeta, StoredVaultConfig};

        let mut state = GlacierState::default();

        // Step 1: load vault configurations
        let vault_names = blobs.list_bucket_configs().await?;
        for vault_name in &vault_names {
            let Ok(json) = blobs.get_bucket_config(vault_name).await else {
                continue;
            };
            let Ok(meta) = serde_json::from_value::<StoredVaultConfig>(json) else {
                continue;
            };
            state.vaults.insert(vault_name.clone(), meta.into_vault());
        }

        // Step 2: load archive metadata
        let version_pairs = blobs.list_version_metas("").await?;
        for (blob_key, archive_id) in version_pairs {
            let Ok(json) = blobs.get_version_meta(&blob_key, &archive_id).await else {
                continue;
            };
            let Ok(meta) = serde_json::from_value::<StoredArchiveMeta>(json) else {
                continue;
            };
            let Some(vault) = state.vaults.get_mut(&meta.vault_name) else {
                continue;
            };
            let archive = meta.into_archive();
            vault.archives.insert(archive.archive_id.clone(), archive);
        }

        Ok(state)
    }

    /// Recover state from the blob store.
    ///
    /// Discovers all `(account_id, region)` pairs stored in the VFS and
    /// recovers each one.
    pub async fn recover(&self) -> Result<(), winterbaume_core::VfsError> {
        let children = self.blobs.list_children().await?;
        for (account_id, region) in &children {
            let scoped = self.blobs.get(account_id, region);
            let recovered = Self::recover_state_from_blobs(&scoped).await?;
            let state = self.state.get(account_id, region);
            *state.write().await = recovered;
        }
        Ok(())
    }

    /// Create a Glacier service backed by the given VFS and restore state
    /// from the metadata stored there.
    pub async fn recover_with_vfs(vfs: Arc<dyn Vfs>) -> Result<Self, winterbaume_core::VfsError> {
        let blobs = BlobStoreMap::new(vfs, "glacier");
        let children = blobs.list_children().await?;
        let mut entries: Vec<((String, String), GlacierState)> = Vec::with_capacity(children.len());
        for (account_id, region) in &children {
            let scoped = blobs.get(account_id, region);
            let recovered = Self::recover_state_from_blobs(&scoped).await?;
            entries.push(((account_id.clone(), region.clone()), recovered));
        }
        Ok(Self {
            state: Arc::new(BackendState::from_iter(entries)),
            blobs: Arc::new(blobs),
            notifier: StateChangeNotifier::new(),
        })
    }

    /// Persist vault configuration to the blob store as a recovery aid.
    /// Errors are silently ignored.
    async fn persist_vault_config(
        &self,
        blobs: &winterbaume_core::BlobStore,
        state: &Arc<RwLock<GlacierState>>,
        vault_name: &str,
    ) {
        let value = {
            let guard = state.read().await;
            guard.vaults.get(vault_name).map(|v| {
                let meta = crate::stored::StoredVaultConfig::from_vault(v);
                serde_json::to_value(&meta).ok()
            })
        };
        if let Some(Some(v)) = value {
            let _ = blobs.put_bucket_config(vault_name, &v).await;
        }
    }

    /// Persist archive metadata to the blob store as a recovery aid.
    /// Errors are silently ignored.
    async fn persist_archive_meta(
        &self,
        blobs: &winterbaume_core::BlobStore,
        vault_name: &str,
        archive: &crate::types::Archive,
    ) {
        let meta = crate::stored::StoredArchiveMeta::from_archive(vault_name, archive);
        if let Ok(v) = serde_json::to_value(&meta) {
            // Use vault_name as the blob_key prefix and archive_id as the version_id
            // in the version-meta namespace.
            let _ = blobs
                .put_version_meta(vault_name, &archive.archive_id, &v)
                .await;
        }
    }

    /// Delete archive metadata from the blob store.
    /// Errors are silently ignored.
    async fn delete_archive_meta(
        &self,
        blobs: &winterbaume_core::BlobStore,
        vault_name: &str,
        archive_id: &str,
    ) {
        let _ = blobs.delete_version_meta(vault_name, archive_id).await;
    }
}

impl MockService for GlacierService {
    fn service_name(&self) -> &str {
        "glacier"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://glacier\..*\.amazonaws\.com",
            r"https?://glacier\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl GlacierService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);
        let blobs = self.blobs.get(account_id, &region);

        let (path, query) = extract_path_and_query(&request.uri);
        let method = request.method.as_str();

        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        // segments[0] = account_id (e.g., "-" or numeric)
        // Dispatch to non-vault routes first.
        if segments.len() >= 3 && segments[1] == "policies" && segments[2] == "data-retrieval" {
            let resp = match method {
                "GET" => self.handle_get_data_retrieval_policy(&state).await,
                "PUT" => {
                    self.handle_set_data_retrieval_policy(&state, &request.body)
                        .await
                }
                _ => glacier_error(405, "MethodNotAllowedException", "Method not allowed"),
            };
            if matches!(method, "PUT") && resp.status / 100 == 2 {
                use winterbaume_core::StatefulService;
                self.notify_state_changed(account_id, &region).await;
            }
            return resp;
        }
        if segments.len() >= 2 && segments[1] == "provisioned-capacity" {
            let resp = match method {
                "GET" => self.handle_list_provisioned_capacity(&state).await,
                "POST" => self.handle_purchase_provisioned_capacity(&state).await,
                _ => glacier_error(405, "MethodNotAllowedException", "Method not allowed"),
            };
            if matches!(method, "POST") && resp.status / 100 == 2 {
                use winterbaume_core::StatefulService;
                self.notify_state_changed(account_id, &region).await;
            }
            return resp;
        }

        if segments.len() < 2 || segments[1] != "vaults" {
            return glacier_error(404, "ResourceNotFoundException", "Not found");
        }

        // Skip account_id (segments[0])
        let after_account = &segments[1..];

        let resp = match (method, after_account) {
            // GET /vaults - ListVaults
            ("GET", [_vaults]) => self.handle_list_vaults(&state).await,
            // PUT /vaults/{name} - CreateVault
            ("PUT", [_vaults, vault_name]) => {
                self.handle_create_vault(&state, &blobs, vault_name, account_id, &region)
                    .await
            }
            // GET /vaults/{name} - DescribeVault
            ("GET", [_vaults, vault_name]) => self.handle_describe_vault(&state, vault_name).await,
            // DELETE /vaults/{name} - DeleteVault
            ("DELETE", [_vaults, vault_name]) => {
                self.handle_delete_vault(&state, &blobs, vault_name).await
            }

            // --- Tags ---
            // POST /vaults/{name}/tags?operation=add - AddTagsToVault
            // POST /vaults/{name}/tags?operation=remove - RemoveTagsFromVault
            ("POST", [_vaults, vault_name, tags]) if *tags == "tags" => {
                let op = query_param(&query, "operation").unwrap_or_default();
                if op == "add" {
                    self.handle_add_tags_to_vault(&state, &blobs, vault_name, &request.body)
                        .await
                } else {
                    self.handle_remove_tags_from_vault(&state, &blobs, vault_name, &request.body)
                        .await
                }
            }
            // GET /vaults/{name}/tags - ListTagsForVault
            ("GET", [_vaults, vault_name, tags]) if *tags == "tags" => {
                self.handle_list_tags_for_vault(&state, vault_name).await
            }

            // --- Access Policy ---
            // PUT /vaults/{name}/access-policy - SetVaultAccessPolicy
            ("PUT", [_vaults, vault_name, ap]) if *ap == "access-policy" => {
                self.handle_set_vault_access_policy(&state, &blobs, vault_name, &request.body)
                    .await
            }
            // GET /vaults/{name}/access-policy - GetVaultAccessPolicy
            ("GET", [_vaults, vault_name, ap]) if *ap == "access-policy" => {
                self.handle_get_vault_access_policy(&state, vault_name)
                    .await
            }
            // DELETE /vaults/{name}/access-policy - DeleteVaultAccessPolicy
            ("DELETE", [_vaults, vault_name, ap]) if *ap == "access-policy" => {
                self.handle_delete_vault_access_policy(&state, &blobs, vault_name)
                    .await
            }

            // --- Notifications ---
            // PUT /vaults/{name}/notification-configuration - SetVaultNotifications
            ("PUT", [_vaults, vault_name, notif]) if *notif == "notification-configuration" => {
                self.handle_set_vault_notifications(&state, &blobs, vault_name, &request.body)
                    .await
            }
            // GET /vaults/{name}/notification-configuration - GetVaultNotifications
            ("GET", [_vaults, vault_name, notif]) if *notif == "notification-configuration" => {
                self.handle_get_vault_notifications(&state, vault_name)
                    .await
            }
            // DELETE /vaults/{name}/notification-configuration - DeleteVaultNotifications
            ("DELETE", [_vaults, vault_name, notif]) if *notif == "notification-configuration" => {
                self.handle_delete_vault_notifications(&state, &blobs, vault_name)
                    .await
            }

            // --- Vault Lock ---
            // POST /vaults/{name}/lock-policy - InitiateVaultLock
            ("POST", [_vaults, vault_name, lock]) if *lock == "lock-policy" => {
                self.handle_initiate_vault_lock(
                    &state,
                    &blobs,
                    vault_name,
                    account_id,
                    &request.body,
                )
                .await
            }
            // GET /vaults/{name}/lock-policy - GetVaultLock
            ("GET", [_vaults, vault_name, lock]) if *lock == "lock-policy" => {
                self.handle_get_vault_lock(&state, vault_name).await
            }
            // DELETE /vaults/{name}/lock-policy - AbortVaultLock
            ("DELETE", [_vaults, vault_name, lock]) if *lock == "lock-policy" => {
                self.handle_abort_vault_lock(&state, &blobs, vault_name)
                    .await
            }
            // POST /vaults/{name}/lock-policy/{lock_id} - CompleteVaultLock
            ("POST", [_vaults, vault_name, lock_policy, lock_id])
                if *lock_policy == "lock-policy" =>
            {
                self.handle_complete_vault_lock(&state, &blobs, vault_name, lock_id)
                    .await
            }

            // --- Archives ---
            // POST /vaults/{name}/archives - UploadArchive
            // self.handle_upload_archive (implemented inline due to async blob I/O)
            ("POST", [_vaults, vault_name, archives]) if *archives == "archives" => {
                let description = request
                    .headers
                    .get("x-amz-archive-description")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("")
                    .to_string();
                // async path - inline here and return early to avoid async/sync mismatch
                let archive_id = uuid::Uuid::new_v4().to_string();
                let sha256 = format!("{:x}", Sha256::digest(&request.body));
                let size = request.body.len();
                let blob_key = format!("{vault_name}/{archive_id}");
                if let Err(e) = blobs.put_from(&blob_key, &mut request.body.as_ref()).await {
                    tracing::error!("Failed to store archive blob: {e:?}");
                    return glacier_error(500, "InternalServerError", "Failed to store archive");
                }
                let upload_result = {
                    let mut st = state.write().await;
                    st.upload_archive(
                        vault_name,
                        archive_id.clone(),
                        sha256.clone(),
                        size,
                        blob_key.clone(),
                        &description,
                    )
                };
                match upload_result {
                    Ok(()) => {
                        // Persist archive metadata for recovery.
                        let archive = crate::types::Archive {
                            archive_id: archive_id.clone(),
                            blob_key,
                            size,
                            sha256: sha256.clone(),
                            description,
                            creation_date: chrono::Utc::now(),
                        };
                        self.persist_archive_meta(&blobs, vault_name, &archive)
                            .await;
                        let mut resp = MockResponse::rest_json(201, "".to_string());
                        resp.headers.insert(
                            HeaderName::from_static("x-amz-archive-id"),
                            archive_id.parse().unwrap(),
                        );
                        resp.headers.insert(
                            HeaderName::from_static("x-amz-sha256-tree-hash"),
                            sha256.parse().unwrap(),
                        );
                        use winterbaume_core::StatefulService;
                        self.notify_state_changed(account_id, &region).await;
                        return resp;
                    }
                    Err(e) => glacier_error_response(&e),
                }
            }
            // DELETE /vaults/{name}/archives/{archive_id} - DeleteArchive
            // self.handle_delete_archive (implemented inline due to async blob I/O)
            ("DELETE", [_vaults, vault_name, _archives, archive_id])
                if after_account[2] == "archives" =>
            {
                let blob_key = {
                    let mut st = state.write().await;
                    match st.delete_archive(vault_name, archive_id) {
                        Ok(key) => key,
                        Err(e) => return glacier_error_response(&e),
                    }
                };
                if let Some(key) = blob_key {
                    let _ = blobs.delete(&key).await;
                }
                self.delete_archive_meta(&blobs, vault_name, archive_id)
                    .await;
                use winterbaume_core::StatefulService;
                self.notify_state_changed(account_id, &region).await;
                return wire::serialize_delete_archive_response();
            }

            // --- Multipart Uploads ---
            // POST /vaults/{name}/multipart-uploads - InitiateMultipartUpload
            ("POST", [_vaults, vault_name, multipart]) if *multipart == "multipart-uploads" => {
                self.handle_initiate_multipart_upload(
                    &state,
                    vault_name,
                    account_id,
                    &request.headers,
                )
                .await
            }
            // GET /vaults/{name}/multipart-uploads - ListMultipartUploads
            ("GET", [_vaults, vault_name, multipart]) if *multipart == "multipart-uploads" => {
                self.handle_list_multipart_uploads(&state, vault_name).await
            }
            // DELETE /vaults/{name}/multipart-uploads/{upload_id} - AbortMultipartUpload
            ("DELETE", [_vaults, vault_name, _multipart, upload_id])
                if after_account[2] == "multipart-uploads" =>
            {
                self.handle_abort_multipart_upload(&state, vault_name, upload_id)
                    .await
            }
            // PUT /vaults/{name}/multipart-uploads/{upload_id} - UploadMultipartPart
            ("PUT", [_vaults, vault_name, _multipart, upload_id])
                if after_account[2] == "multipart-uploads" =>
            {
                self.handle_upload_multipart_part(
                    &state,
                    vault_name,
                    upload_id,
                    &request.headers,
                    &request.body,
                )
                .await
            }
            // POST /vaults/{name}/multipart-uploads/{upload_id} - CompleteMultipartUpload
            ("POST", [_vaults, vault_name, _multipart, upload_id])
                if after_account[2] == "multipart-uploads" =>
            {
                self.handle_complete_multipart_upload(
                    &state,
                    vault_name,
                    upload_id,
                    account_id,
                    &request.headers,
                )
                .await
            }
            // GET /vaults/{name}/multipart-uploads/{upload_id} - ListParts
            ("GET", [_vaults, vault_name, _multipart, upload_id])
                if after_account[2] == "multipart-uploads" =>
            {
                self.handle_list_parts(&state, vault_name, upload_id).await
            }

            // --- Jobs ---
            // POST /vaults/{name}/jobs - InitiateJob
            ("POST", [_vaults, vault_name, jobs]) if *jobs == "jobs" => {
                self.handle_initiate_job(&state, vault_name, account_id, &region, &request.body)
                    .await
            }
            // GET /vaults/{name}/jobs - ListJobs
            ("GET", [_vaults, vault_name, jobs]) if *jobs == "jobs" => {
                self.handle_list_jobs(&state, vault_name).await
            }
            // GET /vaults/{name}/jobs/{job_id}/output - GetJobOutput (must come before DescribeJob)
            // self.handle_get_job_output (implemented inline due to async blob I/O)
            ("GET", [_vaults, vault_name, _jobs, job_id, output])
                if after_account[2] == "jobs" && *output == "output" =>
            {
                // This is async - handle inline
                enum JobResult {
                    ArchiveBlob(String),
                    Inventory(String, Vec<Value>),
                }
                let job_result = {
                    let st = state.read().await;
                    match st.get_job_output(vault_name, job_id) {
                        Ok(Some(JobOutput::ArchiveBody(blob_key))) => {
                            Ok(Some(JobResult::ArchiveBlob(blob_key)))
                        }
                        Ok(Some(JobOutput::Inventory {
                            vault_arn,
                            archives,
                        })) => {
                            let archive_list: Vec<Value> = archives
                                .iter()
                                .map(|a| {
                                    json!({
                                        "ArchiveId": a.archive_id,
                                        "ArchiveDescription": a.description,
                                        "CreationDate": a.creation_date.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                                        "Size": a.size,
                                        "SHA256TreeHash": a.sha256,
                                    })
                                })
                                .collect();
                            Ok(Some(JobResult::Inventory(vault_arn, archive_list)))
                        }
                        Ok(None) => Ok(None),
                        Err(e) => Err(e),
                    }
                };
                return match job_result {
                    Ok(Some(JobResult::ArchiveBlob(blob_key))) => {
                        let mut reader = match blobs.get_reader(&blob_key).await {
                            Ok(r) => r,
                            Err(_) => {
                                return glacier_error(
                                    404,
                                    "ResourceNotFoundException",
                                    "Archive body not found",
                                );
                            }
                        };
                        let mut buf = Vec::new();
                        if let Err(e) = reader.read_to_end(&mut buf).await {
                            return glacier_error(500, "InternalServerError", &e.to_string());
                        }
                        let mut headers = http::HeaderMap::new();
                        headers.insert(
                            http::header::CONTENT_TYPE,
                            "application/octet-stream".parse().unwrap(),
                        );
                        MockResponse {
                            status: 200,
                            headers,
                            body: buf.into(),
                        }
                    }
                    Ok(Some(JobResult::Inventory(vault_arn, archive_list))) => {
                        MockResponse::rest_json(
                            200,
                            json!({
                                "VaultARN": vault_arn,
                                "ArchiveList": archive_list,
                            })
                            .to_string(),
                        )
                    }
                    Ok(None) => glacier_error(404, "ResourceNotFoundException", "Not ready"),
                    Err(e) => glacier_error_response(&e),
                };
            }
            // GET /vaults/{name}/jobs/{job_id} - DescribeJob
            ("GET", [_vaults, vault_name, _jobs, job_id]) if after_account[2] == "jobs" => {
                self.handle_describe_job(&state, vault_name, job_id).await
            }
            _ => glacier_error(404, "ResourceNotFoundException", "Not found"),
        };

        // Notify on mutating methods
        if matches!(method, "PUT" | "POST" | "DELETE") && resp.status / 100 == 2 {
            use winterbaume_core::StatefulService;
            self.notify_state_changed(account_id, &region).await;
        }
        resp
    }

    // --- Vault CRUD ---

    async fn handle_list_vaults(&self, state: &Arc<RwLock<GlacierState>>) -> MockResponse {
        let state = state.read().await;
        let vaults = state.list_vaults();
        let vault_list: Vec<wire::DescribeVaultOutput> =
            vaults.iter().map(|v| vault_to_wire(v)).collect();
        wire::serialize_list_vaults_response(&wire::ListVaultsOutput {
            marker: None,
            vault_list: Some(vault_list),
        })
    }

    async fn handle_create_vault(
        &self,
        state: &Arc<RwLock<GlacierState>>,
        blobs: &winterbaume_core::BlobStore,
        vault_name: &str,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        {
            let mut guard = state.write().await;
            guard.create_vault(vault_name, account_id, region);
        }
        self.persist_vault_config(blobs, state, vault_name).await;
        wire::serialize_create_vault_response(&wire::CreateVaultOutput::default())
    }

    async fn handle_describe_vault(
        &self,
        state: &Arc<RwLock<GlacierState>>,
        vault_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_vault(vault_name) {
            Ok(vault) => wire::serialize_describe_vault_response(&vault_to_wire(vault)),
            Err(e) => glacier_error_response(&e),
        }
    }

    async fn handle_delete_vault(
        &self,
        state: &Arc<RwLock<GlacierState>>,
        blobs: &winterbaume_core::BlobStore,
        vault_name: &str,
    ) -> MockResponse {
        {
            let mut guard = state.write().await;
            guard.delete_vault(vault_name);
        }
        let _ = blobs.delete_bucket_config(vault_name).await;
        wire::serialize_delete_vault_response()
    }

    // --- Tags ---

    async fn handle_add_tags_to_vault(
        &self,
        state: &Arc<RwLock<GlacierState>>,
        blobs: &winterbaume_core::BlobStore,
        vault_name: &str,
        body: &bytes::Bytes,
    ) -> MockResponse {
        let body: Value = serde_json::from_slice(body).unwrap_or(json!({}));
        let tags: HashMap<String, String> = body
            .get("Tags")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();
        let result = {
            let mut guard = state.write().await;
            guard.add_tags_to_vault(vault_name, tags)
        };
        match result {
            Ok(()) => {
                self.persist_vault_config(blobs, state, vault_name).await;
                wire::serialize_add_tags_to_vault_response()
            }
            Err(e) => glacier_error_response(&e),
        }
    }

    async fn handle_remove_tags_from_vault(
        &self,
        state: &Arc<RwLock<GlacierState>>,
        blobs: &winterbaume_core::BlobStore,
        vault_name: &str,
        body: &bytes::Bytes,
    ) -> MockResponse {
        let body: Value = serde_json::from_slice(body).unwrap_or(json!({}));
        let tag_keys: Vec<String> = body
            .get("TagKeys")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();
        let result = {
            let mut guard = state.write().await;
            guard.remove_tags_from_vault(vault_name, &tag_keys)
        };
        match result {
            Ok(()) => {
                self.persist_vault_config(blobs, state, vault_name).await;
                wire::serialize_remove_tags_from_vault_response()
            }
            Err(e) => glacier_error_response(&e),
        }
    }

    async fn handle_list_tags_for_vault(
        &self,
        state: &Arc<RwLock<GlacierState>>,
        vault_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_tags_for_vault(vault_name) {
            Ok(tags) => {
                wire::serialize_list_tags_for_vault_response(&wire::ListTagsForVaultOutput {
                    tags: Some(tags),
                })
            }
            Err(e) => glacier_error_response(&e),
        }
    }

    // --- Access Policy ---

    async fn handle_set_vault_access_policy(
        &self,
        state: &Arc<RwLock<GlacierState>>,
        blobs: &winterbaume_core::BlobStore,
        vault_name: &str,
        body: &bytes::Bytes,
    ) -> MockResponse {
        // VaultAccessPolicy is the httpPayload - body is {"Policy": "..."}
        let body: Value = serde_json::from_slice(body).unwrap_or(json!({}));
        let policy_str = body
            .get("Policy")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let result = {
            let mut guard = state.write().await;
            guard.set_vault_access_policy(vault_name, policy_str)
        };
        match result {
            Ok(()) => {
                self.persist_vault_config(blobs, state, vault_name).await;
                wire::serialize_set_vault_access_policy_response()
            }
            Err(e) => glacier_error_response(&e),
        }
    }

    async fn handle_get_vault_access_policy(
        &self,
        state: &Arc<RwLock<GlacierState>>,
        vault_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_vault_access_policy(vault_name) {
            Ok(policy_str) => {
                // Response body is VaultAccessPolicy directly (httpPayload)
                let body = serde_json::to_string(&wire::VaultAccessPolicy { policy: policy_str })
                    .unwrap_or_else(|_| "{}".to_string());
                MockResponse::rest_json(200, body)
            }
            Err(e) => glacier_error_response(&e),
        }
    }

    async fn handle_delete_vault_access_policy(
        &self,
        state: &Arc<RwLock<GlacierState>>,
        blobs: &winterbaume_core::BlobStore,
        vault_name: &str,
    ) -> MockResponse {
        let result = {
            let mut guard = state.write().await;
            guard.delete_vault_access_policy(vault_name)
        };
        match result {
            Ok(()) => {
                self.persist_vault_config(blobs, state, vault_name).await;
                wire::serialize_delete_vault_access_policy_response()
            }
            Err(e) => glacier_error_response(&e),
        }
    }

    // --- Notifications ---

    async fn handle_set_vault_notifications(
        &self,
        state: &Arc<RwLock<GlacierState>>,
        blobs: &winterbaume_core::BlobStore,
        vault_name: &str,
        body: &bytes::Bytes,
    ) -> MockResponse {
        // VaultNotificationConfig is the httpPayload - body is {"SNSTopic": ..., "Events": [...]}
        let body: Value = serde_json::from_slice(body).unwrap_or(json!({}));
        let sns_topic = body
            .get("SNSTopic")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let events: Vec<String> = body
            .get("Events")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();
        let result = {
            let mut guard = state.write().await;
            guard.set_vault_notifications(vault_name, sns_topic, events)
        };
        match result {
            Ok(()) => {
                self.persist_vault_config(blobs, state, vault_name).await;
                wire::serialize_set_vault_notifications_response()
            }
            Err(e) => glacier_error_response(&e),
        }
    }

    async fn handle_get_vault_notifications(
        &self,
        state: &Arc<RwLock<GlacierState>>,
        vault_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_vault_notifications(vault_name) {
            Ok(config) => match config {
                Some(c) => {
                    // Response body is VaultNotificationConfig directly (httpPayload)
                    let wire_config = wire::VaultNotificationConfig {
                        events: Some(c.events),
                        s_n_s_topic: c.sns_topic,
                    };
                    let body =
                        serde_json::to_string(&wire_config).unwrap_or_else(|_| "{}".to_string());
                    MockResponse::rest_json(200, body)
                }
                // No config - return 200 with empty body so SDK parses it as None
                None => MockResponse {
                    status: 200,
                    headers: http::HeaderMap::new(),
                    body: bytes::Bytes::new(),
                },
            },
            Err(e) => glacier_error_response(&e),
        }
    }

    async fn handle_delete_vault_notifications(
        &self,
        state: &Arc<RwLock<GlacierState>>,
        blobs: &winterbaume_core::BlobStore,
        vault_name: &str,
    ) -> MockResponse {
        let result = {
            let mut guard = state.write().await;
            guard.delete_vault_notifications(vault_name)
        };
        match result {
            Ok(()) => {
                self.persist_vault_config(blobs, state, vault_name).await;
                wire::serialize_delete_vault_notifications_response()
            }
            Err(e) => glacier_error_response(&e),
        }
    }

    // --- Vault Lock ---

    async fn handle_initiate_vault_lock(
        &self,
        state: &Arc<RwLock<GlacierState>>,
        blobs: &winterbaume_core::BlobStore,
        vault_name: &str,
        account_id: &str,
        body: &bytes::Bytes,
    ) -> MockResponse {
        // VaultLockPolicy is the httpPayload - body is {"Policy": "..."}
        let body: Value = serde_json::from_slice(body).unwrap_or(json!({}));
        let policy = body
            .get("Policy")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let lock_result = {
            let mut guard = state.write().await;
            guard.initiate_vault_lock(vault_name, policy)
        };
        match lock_result {
            Ok(lock_id) => {
                self.persist_vault_config(blobs, state, vault_name).await;
                let location = format!("/{account_id}/vaults/{vault_name}/lock-policy/{lock_id}");
                let mut resp =
                    wire::serialize_initiate_vault_lock_response(&wire::InitiateVaultLockOutput {
                        lock_id: Some(lock_id.clone()),
                    });
                resp.headers
                    .insert(http::header::LOCATION, location.parse().unwrap());
                resp.headers.insert(
                    HeaderName::from_static("x-amz-lock-id"),
                    lock_id.parse().unwrap(),
                );
                resp
            }
            Err(e) => glacier_error_response(&e),
        }
    }

    async fn handle_get_vault_lock(
        &self,
        state: &Arc<RwLock<GlacierState>>,
        vault_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_vault_lock(vault_name) {
            Ok(lock_opt) => {
                let output = match lock_opt {
                    Some(lock) => wire::GetVaultLockOutput {
                        creation_date: Some(
                            lock.creation_date
                                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                                .to_string(),
                        ),
                        expiration_date: Some(
                            (lock.creation_date + chrono::TimeDelta::hours(24))
                                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                                .to_string(),
                        ),
                        policy: lock.policy.clone(),
                        state: Some(match lock.state {
                            crate::types::VaultLockState::InProgress => "InProgress".to_string(),
                            crate::types::VaultLockState::Locked => "Locked".to_string(),
                        }),
                    },
                    None => wire::GetVaultLockOutput::default(),
                };
                wire::serialize_get_vault_lock_response(&output)
            }
            Err(e) => glacier_error_response(&e),
        }
    }

    async fn handle_abort_vault_lock(
        &self,
        state: &Arc<RwLock<GlacierState>>,
        blobs: &winterbaume_core::BlobStore,
        vault_name: &str,
    ) -> MockResponse {
        let result = {
            let mut guard = state.write().await;
            guard.abort_vault_lock(vault_name)
        };
        match result {
            Ok(()) => {
                self.persist_vault_config(blobs, state, vault_name).await;
                wire::serialize_abort_vault_lock_response()
            }
            Err(e) => glacier_error_response(&e),
        }
    }

    async fn handle_complete_vault_lock(
        &self,
        state: &Arc<RwLock<GlacierState>>,
        blobs: &winterbaume_core::BlobStore,
        vault_name: &str,
        lock_id: &str,
    ) -> MockResponse {
        let result = {
            let mut guard = state.write().await;
            guard.complete_vault_lock(vault_name, lock_id)
        };
        match result {
            Ok(()) => {
                self.persist_vault_config(blobs, state, vault_name).await;
                wire::serialize_complete_vault_lock_response()
            }
            Err(e) => glacier_error_response(&e),
        }
    }

    // --- Multipart Uploads ---

    async fn handle_initiate_multipart_upload(
        &self,
        state: &Arc<RwLock<GlacierState>>,
        vault_name: &str,
        account_id: &str,
        headers: &http::HeaderMap,
    ) -> MockResponse {
        let part_size: Option<i64> = headers
            .get("x-amz-part-size")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok());
        let description: Option<String> = headers
            .get("x-amz-archive-description")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());
        let vault_arn = {
            let st = state.read().await;
            match st.describe_vault(vault_name) {
                Ok(v) => v.arn.clone(),
                Err(e) => return glacier_error_response(&e),
            }
        };
        let mut st = state.write().await;
        match st.initiate_multipart_upload(vault_name, &vault_arn, description, part_size) {
            Ok(upload_id) => {
                let location =
                    format!("/{account_id}/vaults/{vault_name}/multipart-uploads/{upload_id}");
                let mut resp = wire::serialize_initiate_multipart_upload_response(
                    &wire::InitiateMultipartUploadOutput {
                        location: Some(location.clone()),
                        upload_id: Some(upload_id.clone()),
                    },
                );
                resp.headers
                    .insert(http::header::LOCATION, location.parse().unwrap());
                resp.headers.insert(
                    HeaderName::from_static("x-amz-multipart-upload-id"),
                    upload_id.parse().unwrap(),
                );
                resp
            }
            Err(e) => glacier_error_response(&e),
        }
    }

    async fn handle_list_multipart_uploads(
        &self,
        state: &Arc<RwLock<GlacierState>>,
        vault_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_multipart_uploads(vault_name) {
            Ok(uploads) => {
                let uploads_list: Vec<wire::UploadListElement> = uploads
                    .iter()
                    .map(|u| wire::UploadListElement {
                        archive_description: u.description.clone(),
                        creation_date: Some(
                            u.creation_date.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                        ),
                        multipart_upload_id: Some(u.upload_id.clone()),
                        part_size_in_bytes: u.part_size,
                        vault_a_r_n: Some(u.vault_arn.clone()),
                    })
                    .collect();
                wire::serialize_list_multipart_uploads_response(&wire::ListMultipartUploadsOutput {
                    marker: None,
                    uploads_list: Some(uploads_list),
                })
            }
            Err(e) => glacier_error_response(&e),
        }
    }

    async fn handle_abort_multipart_upload(
        &self,
        state: &Arc<RwLock<GlacierState>>,
        vault_name: &str,
        upload_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.abort_multipart_upload(vault_name, upload_id) {
            Ok(()) => wire::serialize_abort_multipart_upload_response(),
            Err(e) => glacier_error_response(&e),
        }
    }

    async fn handle_upload_multipart_part(
        &self,
        state: &Arc<RwLock<GlacierState>>,
        vault_name: &str,
        upload_id: &str,
        headers: &http::HeaderMap,
        body: &bytes::Bytes,
    ) -> MockResponse {
        let range: Option<String> = headers
            .get("content-range")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());
        let sha256 = format!("{:x}", Sha256::digest(body));
        let checksum = sha256.clone();
        let mut state = state.write().await;
        match state.upload_multipart_part(vault_name, upload_id, range, sha256) {
            Ok(()) => {
                let mut resp = wire::serialize_upload_multipart_part_response(
                    &wire::UploadMultipartPartOutput {
                        checksum: Some(checksum.clone()),
                    },
                );
                resp.headers.insert(
                    HeaderName::from_static("x-amz-sha256-tree-hash"),
                    checksum.parse().unwrap(),
                );
                resp
            }
            Err(e) => glacier_error_response(&e),
        }
    }

    async fn handle_complete_multipart_upload(
        &self,
        state: &Arc<RwLock<GlacierState>>,
        vault_name: &str,
        upload_id: &str,
        account_id: &str,
        headers: &http::HeaderMap,
    ) -> MockResponse {
        let archive_size: Option<i64> = headers
            .get("x-amz-archive-size")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok());
        let checksum: Option<String> = headers
            .get("x-amz-sha256-tree-hash")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());
        let mut state = state.write().await;
        match state.complete_multipart_upload(vault_name, upload_id, archive_size, checksum.clone())
        {
            Ok(archive_id) => {
                let location = format!("/{account_id}/vaults/{vault_name}/archives/{archive_id}");
                let mut resp = wire::serialize_complete_multipart_upload_response(
                    &wire::ArchiveCreationOutput {
                        archive_id: Some(archive_id.clone()),
                        checksum,
                        location: Some(location.clone()),
                    },
                );
                resp.headers
                    .insert(http::header::LOCATION, location.parse().unwrap());
                resp.headers.insert(
                    HeaderName::from_static("x-amz-archive-id"),
                    archive_id.parse().unwrap(),
                );
                resp
            }
            Err(e) => glacier_error_response(&e),
        }
    }

    async fn handle_list_parts(
        &self,
        state: &Arc<RwLock<GlacierState>>,
        vault_name: &str,
        upload_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_parts(vault_name, upload_id) {
            Ok(upload) => {
                let parts: Vec<wire::PartListElement> = upload
                    .parts
                    .values()
                    .map(|p| wire::PartListElement {
                        range_in_bytes: Some(p.range_in_bytes.clone()),
                        s_h_a256_tree_hash: Some(p.sha256.clone()),
                    })
                    .collect();
                wire::serialize_list_parts_response(&wire::ListPartsOutput {
                    archive_description: upload.description.clone(),
                    creation_date: Some(
                        upload
                            .creation_date
                            .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                            .to_string(),
                    ),
                    marker: None,
                    multipart_upload_id: Some(upload.upload_id.clone()),
                    part_size_in_bytes: upload.part_size,
                    parts: Some(parts),
                    vault_a_r_n: Some(upload.vault_arn.clone()),
                })
            }
            Err(e) => glacier_error_response(&e),
        }
    }

    // --- Jobs ---

    async fn handle_initiate_job(
        &self,
        state: &Arc<RwLock<GlacierState>>,
        vault_name: &str,
        account_id: &str,
        _region: &str,
        body: &bytes::Bytes,
    ) -> MockResponse {
        // JobParameters is the httpPayload - body IS the JobParameters struct directly
        let body: Value = match serde_json::from_slice(body) {
            Ok(v) => v,
            Err(_) => return glacier_error(400, "InvalidParameterException", "Invalid JSON"),
        };
        let job_type = body.get("Type").and_then(|v| v.as_str()).unwrap_or("");
        let archive_id = body.get("ArchiveId").and_then(|v| v.as_str());
        let tier = body
            .get("Tier")
            .and_then(|v| v.as_str())
            .unwrap_or("Standard");

        let mut state = state.write().await;
        match state.initiate_job(vault_name, job_type, tier, archive_id) {
            Ok(job_id) => {
                let location = format!("/{account_id}/vaults/{vault_name}/jobs/{job_id}");
                let mut resp = wire::serialize_initiate_job_response(&wire::InitiateJobOutput {
                    job_id: Some(job_id.clone()),
                    location: Some(location.clone()),
                    job_output_path: None,
                });
                resp.headers.insert(
                    HeaderName::from_static("x-amz-job-id"),
                    job_id.parse().unwrap(),
                );
                resp.headers
                    .insert(http::header::LOCATION, location.parse().unwrap());
                resp
            }
            Err(e) => glacier_error_response(&e),
        }
    }

    async fn handle_list_jobs(
        &self,
        state: &Arc<RwLock<GlacierState>>,
        vault_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_jobs(vault_name) {
            Ok(jobs) => {
                let job_list: Vec<wire::GlacierJobDescription> =
                    jobs.iter().map(|j| job_to_wire(j)).collect();
                wire::serialize_list_jobs_response(&wire::ListJobsOutput {
                    job_list: Some(job_list),
                    marker: None,
                })
            }
            Err(e) => glacier_error_response(&e),
        }
    }

    async fn handle_describe_job(
        &self,
        state: &Arc<RwLock<GlacierState>>,
        vault_name: &str,
        job_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_job(vault_name, job_id) {
            Ok(job) => wire::serialize_describe_job_response(&job_to_wire(job)),
            Err(e) => glacier_error_response(&e),
        }
    }

    // --- Data Retrieval Policy ---

    async fn handle_get_data_retrieval_policy(
        &self,
        state: &Arc<RwLock<GlacierState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let policy = state.get_data_retrieval_policy();
        let wire_policy = policy.map(|p| wire::DataRetrievalPolicy {
            rules: Some(
                p.rules
                    .iter()
                    .map(|r| wire::DataRetrievalRule {
                        bytes_per_hour: r.bytes_per_hour,
                        strategy: r.strategy.clone(),
                    })
                    .collect(),
            ),
        });
        wire::serialize_get_data_retrieval_policy_response(&wire::GetDataRetrievalPolicyOutput {
            policy: wire_policy,
        })
    }

    async fn handle_set_data_retrieval_policy(
        &self,
        state: &Arc<RwLock<GlacierState>>,
        body: &bytes::Bytes,
    ) -> MockResponse {
        let body_val: Value = serde_json::from_slice(body).unwrap_or(json!({}));
        let policy = body_val.get("Policy").and_then(|p| {
            let rules_val = p.get("Rules")?;
            let rules: Vec<DataRetrievalRule> = rules_val
                .as_array()?
                .iter()
                .map(|r| DataRetrievalRule {
                    bytes_per_hour: r.get("BytesPerHour").and_then(|v| v.as_i64()),
                    strategy: r
                        .get("Strategy")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                })
                .collect();
            Some(DataRetrievalPolicy { rules })
        });
        let mut state = state.write().await;
        state.set_data_retrieval_policy(policy);
        wire::serialize_set_data_retrieval_policy_response()
    }

    // --- Provisioned Capacity ---

    async fn handle_list_provisioned_capacity(
        &self,
        state: &Arc<RwLock<GlacierState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let caps = state.list_provisioned_capacity();
        let list: Vec<wire::ProvisionedCapacityDescription> = caps
            .iter()
            .map(|c| wire::ProvisionedCapacityDescription {
                capacity_id: Some(c.capacity_id.clone()),
                expiration_date: Some(
                    c.expiration_date
                        .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                        .to_string(),
                ),
                start_date: Some(c.start_date.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()),
            })
            .collect();
        wire::serialize_list_provisioned_capacity_response(&wire::ListProvisionedCapacityOutput {
            provisioned_capacity_list: Some(list),
        })
    }

    async fn handle_purchase_provisioned_capacity(
        &self,
        state: &Arc<RwLock<GlacierState>>,
    ) -> MockResponse {
        let mut state = state.write().await;
        let capacity_id = state.purchase_provisioned_capacity();
        // capacityId is returned in x-amz-capacity-id header (httpHeader in Smithy model)
        let mut resp = wire::serialize_purchase_provisioned_capacity_response(
            &wire::PurchaseProvisionedCapacityOutput {
                capacity_id: Some(capacity_id.clone()),
            },
        );
        resp.headers.insert(
            HeaderName::from_static("x-amz-capacity-id"),
            capacity_id.parse().unwrap(),
        );
        resp
    }
}

fn vault_to_wire(vault: &crate::types::Vault) -> wire::DescribeVaultOutput {
    let archives_size: usize = vault.archives.values().map(|a| a.size).sum();
    wire::DescribeVaultOutput {
        creation_date: Some(
            vault
                .created_at
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        ),
        last_inventory_date: Some(
            vault
                .created_at
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        ),
        number_of_archives: Some(vault.archives.len() as i64),
        size_in_bytes: Some(archives_size as i64),
        vault_a_r_n: Some(vault.arn.clone()),
        vault_name: Some(vault.vault_name.clone()),
    }
}

fn job_to_wire(job: &crate::types::Job) -> wire::GlacierJobDescription {
    let action = match job.job_type {
        JobType::ArchiveRetrieval => "ArchiveRetrieval",
        JobType::InventoryRetrieval => "InventoryRetrieval",
    };

    let completed = job.is_completed();
    let status_code = if completed { "Succeeded" } else { "InProgress" };
    let inventory_size = if completed { 10000 } else { 0 };

    let completion_date = if completed {
        Some(
            job.completed_at
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        )
    } else {
        None
    };

    wire::GlacierJobDescription {
        action: Some(action.to_string()),
        archive_id: job.archive_id.clone(),
        archive_s_h_a256_tree_hash: None,
        archive_size_in_bytes: Some(0),
        completed: Some(completed),
        completion_date,
        creation_date: Some(job.created_at.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()),
        inventory_retrieval_parameters: None,
        inventory_size_in_bytes: Some(inventory_size),
        job_description: None,
        job_id: Some(job.job_id.clone()),
        job_output_path: None,
        output_location: None,
        retrieval_byte_range: None,
        s_h_a256_tree_hash: None,
        s_n_s_topic: None,
        select_parameters: None,
        status_code: Some(status_code.to_string()),
        status_message: None,
        tier: Some(job.tier.clone()),
        vault_a_r_n: Some(job.vault_arn.clone()),
    }
}

fn extract_path_and_query(uri: &str) -> (String, String) {
    let after_host = if let Some(idx) = uri.find("amazonaws.com") {
        &uri[idx + "amazonaws.com".len()..]
    } else {
        uri
    };
    if let Some(q) = after_host.find('?') {
        (after_host[..q].to_string(), after_host[q + 1..].to_string())
    } else {
        (after_host.to_string(), String::new())
    }
}

fn query_param<'a>(query: &'a str, key: &str) -> Option<&'a str> {
    for pair in query.split('&') {
        if let Some((k, v)) = pair.split_once('=') {
            if k == key {
                return Some(v);
            }
        }
    }
    None
}

fn glacier_error_response(err: &GlacierError) -> MockResponse {
    let (status, error_type) = match err {
        GlacierError::VaultNotFound => (404, "VaultNotFound"),
        GlacierError::ResourceNotFound => (404, "ResourceNotFoundException"),
        GlacierError::JobNotFound => (404, "ResourceNotFoundException"),
        GlacierError::LockNotFound => (404, "ResourceNotFoundException"),
        GlacierError::MultipartUploadNotFound => (404, "ResourceNotFoundException"),
    };
    glacier_error(status, error_type, &err.to_string())
}

fn glacier_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "code": code,
        "message": message,
        "type": "User",
    });
    let mut headers = http::HeaderMap::new();
    headers.insert(
        http::header::CONTENT_TYPE,
        "application/json".parse().unwrap(),
    );
    headers.insert(
        HeaderName::from_static("x-amzn-errortype"),
        code.parse().unwrap(),
    );
    MockResponse {
        status,
        headers,
        body: body.to_string().into(),
    }
}

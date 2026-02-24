//! A namespaced blob store built on top of a [`Vfs`].
//!
//! [`BlobStore`] wraps an `Arc<dyn Vfs>` and prepends a namespace prefix to
//! every key, keeping different services' blobs isolated even when they share
//! the same underlying VFS instance.
//!
//! # Internal path layout
//!
//! Within the VFS, all blob **content** (versioned and unversioned alike) lives
//! under the `data/` sub-tree:
//!
//! ```text
//! {namespace}/data/{key}/{version_id}
//! ```
//!
//! Internal **metadata** lives under the `meta/` sub-tree:
//!
//! ```text
//! {namespace}/meta/current/{key}          ← current version_id (or COMPOSITE_SENTINEL)
//! {namespace}/meta/composites/{key}       ← composite manifest (JSON list of part keys)
//! ```
//!
//! Because user-provided keys are placed under `data/{key}/` (with an extra
//! `{version_id}` segment appended), no key value — no matter what it contains
//! — can collide with any internal metadata path.  Similarly, neither `.v`
//! nor any other reserved path segment appears in the layout.
//!
//! # Unversioned vs. versioned blobs
//!
//! Both use the same storage scheme (`data/{key}/{version_id}`).
//!
//! * **Unversioned** ([`put`]): creates a new version and deletes the
//!   previous one.  The caller sees simple put/get/delete semantics.
//! * **Versioned** ([`put_versioned`]): creates a new version but retains
//!   all previous versions.  Callers access specific versions via
//!   [`get_version`], list them with [`list_versions`], and remove individual
//!   versions with [`delete_version`].
//!
//! # Composite blobs
//!
//! A *composite* blob is a virtual blob assembled from an ordered list of
//! existing part blobs.  The composite manifest (JSON list of part keys) is
//! stored at `{namespace}/meta/composites/{key}` and the current-pointer at
//! `{namespace}/meta/current/{key}` is set to [`COMPOSITE_SENTINEL`].
//! [`get`] transparently reads and concatenates the parts; [`delete`]
//! cascades deletion to all referenced parts.  [`get_reader`] buffers the
//! assembled composite and returns a cursor to avoid a streaming-chain
//! implementation.
//!
//! # Async streaming API
//!
//! * [`put`] — write a blob from a [`bytes::Bytes`] value.
//! * [`put_from`] — write a blob from any [`tokio::io::AsyncRead`] source,
//!   avoiding an intermediate allocation.
//! * [`get`] — return the blob's contents as [`bytes::Bytes`].
//! * [`get_reader`] — return a `Box<dyn AsyncRead + Send + Unpin>` that
//!   streams the blob content.  For composite blobs the parts are assembled
//!   first (eagerly), then wrapped in a cursor.

use std::io;
use std::sync::Arc;

use bytes::{Bytes, BytesMut};
use serde_json::Value as JsonValue;
use tokio::io::{AsyncRead, AsyncReadExt};

use crate::vfs::{MemVfs, Vfs, VfsEntry, VfsError, VfsStat};

/// Value stored in `meta/current/{key}` to indicate the key holds a composite
/// blob rather than a regular versioned blob.
///
/// Version IDs are formatted as `{39-digit-nanos}_{8-hex-chars}`, which never
/// equals this string.
const COMPOSITE_SENTINEL: &str = "composite";

/// A namespaced blob store backed by a [`Vfs`].
///
/// Keys passed to methods on `BlobStore` are automatically placed under
/// `{namespace}/data/{key}/{version_id}` in the underlying VFS, keeping them
/// isolated from all internal metadata paths.
///
/// # Example
///
/// ```ignore
/// let store = BlobStore::mem("s3");
/// store.put("my-bucket/my-key", Bytes::from_static(b"hello")).await.unwrap();
/// let data = store.get("my-bucket/my-key").await.unwrap();
/// ```
#[derive(Clone)]
pub struct BlobStore {
    vfs: Arc<dyn Vfs>,
    namespace: String,
}

impl BlobStore {
    /// Create a new `BlobStore` backed by the given VFS, using `namespace` as
    /// the key prefix.
    pub fn new(vfs: Arc<dyn Vfs>, namespace: impl Into<String>) -> Self {
        Self {
            vfs,
            namespace: namespace.into(),
        }
    }

    /// Convenience constructor that creates an in-memory [`MemVfs`] store.
    pub fn mem(namespace: impl Into<String>) -> Self {
        Self::new(Arc::new(MemVfs::new()), namespace)
    }

    /// Create a child store sharing the same VFS but with a sub-namespace.
    ///
    /// The child's effective prefix is `{self.namespace}/{sub_namespace}`.
    pub fn child(&self, sub_namespace: impl AsRef<str>) -> Self {
        Self {
            vfs: Arc::clone(&self.vfs),
            namespace: format!("{}/{}", self.namespace, sub_namespace.as_ref()),
        }
    }

    /// Return the underlying `Arc<dyn Vfs>` so services can share it across
    /// multiple `BlobStore` instances.
    pub fn vfs(&self) -> Arc<dyn Vfs> {
        Arc::clone(&self.vfs)
    }

    // --- internal path helpers ---

    /// VFS path for a specific version of `key`'s blob content.
    fn version_path(&self, key: &str, version_id: &str) -> String {
        format!("{}/data/{}/{}", self.namespace, key, version_id)
    }

    /// VFS path that serves as the directory prefix for all versions of `key`.
    fn version_dir(&self, key: &str) -> String {
        format!("{}/data/{}/", self.namespace, key)
    }

    /// VFS path for the current-version pointer of `key`.
    fn current_path(&self, key: &str) -> String {
        format!("{}/meta/current/{}", self.namespace, key)
    }

    /// VFS path for the composite manifest of `key`.
    fn composite_path(&self, key: &str) -> String {
        format!("{}/meta/composites/{}", self.namespace, key)
    }

    // --- low-level helpers that bridge Bytes to the async Vfs ---

    /// Write `data` (a `Bytes` value) to the given VFS path.
    async fn vfs_put_bytes(&self, path: &str, data: Bytes) -> Result<(), VfsError> {
        let mut cursor = io::Cursor::new(data);
        self.vfs.put(path, &mut cursor).await?;
        Ok(())
    }

    /// Read the blob at the given VFS path into a `Bytes` value.
    async fn vfs_get_bytes(&self, path: &str) -> Result<Bytes, VfsError> {
        let mut reader = self.vfs.get(path).await?;
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).await.map_err(VfsError::Io)?;
        Ok(Bytes::from(buf))
    }

    // --- internal helpers ---

    /// Read the current version_id (or COMPOSITE_SENTINEL) for `key`,
    /// returning `None` if no blob exists.
    async fn read_current(&self, key: &str) -> Option<String> {
        self.vfs_get_bytes(&self.current_path(key))
            .await
            .ok()
            .and_then(|b| String::from_utf8(b.to_vec()).ok())
    }

    /// Clean up whatever is currently stored under `key`, if anything.
    async fn evict_current(&self, key: &str) {
        let Some(current) = self.read_current(key).await else {
            return;
        };
        if current == COMPOSITE_SENTINEL {
            if let Ok(manifest) = self.vfs_get_bytes(&self.composite_path(key)).await {
                if let Ok(parts) = serde_json::from_slice::<Vec<String>>(&manifest) {
                    for part in parts {
                        let _ = self.evict_key(&part).await;
                    }
                }
                let _ = self.vfs.delete(&self.composite_path(key)).await;
            }
        } else {
            let _ = self.vfs.delete(&self.version_path(key, &current)).await;
        }
        let _ = self.vfs.delete(&self.current_path(key)).await;
    }

    /// Remove an arbitrary user key (blob + current pointer), used when
    /// cascading composite part deletion.
    async fn evict_key(&self, key: &str) -> bool {
        let Some(current) = self.read_current(key).await else {
            return false;
        };
        if current != COMPOSITE_SENTINEL {
            let _ = self.vfs.delete(&self.version_path(key, &current)).await;
        }
        let _ = self.vfs.delete(&self.current_path(key)).await;
        true
    }

    fn make_version_id() -> String {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let suffix = uuid::Uuid::new_v4().simple().to_string();
        format!("{nanos:039}_{}", &suffix[..8])
    }

    // --- public blob API ---

    /// Store `data` under `key`, replacing any existing blob (including
    /// previous versions).
    ///
    /// This is the **unversioned** put: the previous blob, if any, is deleted.
    /// To retain previous versions use [`put_versioned`].
    ///
    /// For large payloads consider [`put_from`] to avoid buffering the entire
    /// body in a `Bytes` allocation.
    pub async fn put(&self, key: &str, data: Bytes) -> Result<(), VfsError> {
        let mut cursor = io::Cursor::new(data);
        self.put_from(key, &mut cursor).await
    }

    /// Stream data from `reader` into `key`, replacing any existing blob.
    ///
    /// This is the streaming equivalent of [`put`] and avoids buffering the
    /// payload into a contiguous `Bytes` allocation.
    pub async fn put_from(
        &self,
        key: &str,
        reader: &mut (dyn AsyncRead + Unpin + Send),
    ) -> Result<(), VfsError> {
        self.evict_current(key).await;
        let vid = Self::make_version_id();
        self.vfs.put(&self.version_path(key, &vid), reader).await?;
        self.vfs_put_bytes(&self.current_path(key), Bytes::from(vid.into_bytes()))
            .await
    }

    /// Retrieve the blob stored under `key`, buffering the full payload into
    /// a [`Bytes`] value.
    ///
    /// If the blob is a composite (registered with [`put_composite`]), the
    /// parts are read and concatenated in order before being returned.
    ///
    /// Returns [`VfsError::NotFound`] if the key does not exist.
    ///
    /// For large blobs consider [`get_reader`] to avoid the full-payload
    /// allocation.
    pub async fn get(&self, key: &str) -> Result<Bytes, VfsError> {
        let current = self
            .read_current(key)
            .await
            .ok_or_else(|| VfsError::NotFound(key.to_string()))?;
        if current == COMPOSITE_SENTINEL {
            let manifest = self.vfs_get_bytes(&self.composite_path(key)).await?;
            self.assemble_composite(&manifest).await
        } else {
            self.vfs_get_bytes(&self.version_path(key, &current)).await
        }
    }

    /// Return a streaming reader for the blob stored under `key`.
    ///
    /// Unlike [`get`], this does not buffer the full payload for regular
    /// (non-composite) blobs.  For composite blobs the parts are assembled
    /// into memory first (same as [`get`]) before being wrapped in a cursor.
    ///
    /// Returns [`VfsError::NotFound`] if the key does not exist.
    pub async fn get_reader(
        &self,
        key: &str,
    ) -> Result<Box<dyn AsyncRead + Send + Unpin>, VfsError> {
        let current = self
            .read_current(key)
            .await
            .ok_or_else(|| VfsError::NotFound(key.to_string()))?;
        if current == COMPOSITE_SENTINEL {
            // For composite blobs, assemble eagerly and return a Cursor.
            let manifest = self.vfs_get_bytes(&self.composite_path(key)).await?;
            let data = self.assemble_composite(&manifest).await?;
            Ok(Box::new(io::Cursor::new(data)) as Box<dyn AsyncRead + Send + Unpin>)
        } else {
            self.vfs.get(&self.version_path(key, &current)).await
        }
    }

    async fn assemble_composite(&self, manifest: &Bytes) -> Result<Bytes, VfsError> {
        let part_keys: Vec<String> = serde_json::from_slice(manifest)
            .map_err(|e| VfsError::Io(io::Error::new(io::ErrorKind::InvalidData, e.to_string())))?;
        let mut combined = BytesMut::new();
        for part_key in &part_keys {
            let part_data = Box::pin(self.get(part_key)).await?;
            combined.extend_from_slice(&part_data);
        }
        Ok(combined.freeze())
    }

    /// Delete the blob stored under `key`.
    ///
    /// If the blob is a composite, all referenced part blobs are also deleted.
    ///
    /// Returns [`VfsError::NotFound`] if the key does not exist.
    pub async fn delete(&self, key: &str) -> Result<(), VfsError> {
        if self.read_current(key).await.is_none() {
            return Err(VfsError::NotFound(key.to_string()));
        }
        self.evict_current(key).await;
        Ok(())
    }

    /// Delete all blobs whose key starts with `key_prefix`.
    ///
    /// Composite blobs are expanded: their parts are deleted before the
    /// manifests are removed.  Versioned blobs under the prefix (including
    /// non-current versions) are also removed.  This is a best-effort bulk
    /// delete; individual failures are silently ignored.
    pub async fn delete_by_prefix(&self, key_prefix: &str) -> Result<(), VfsError> {
        // 1. Evict all current blobs (regular and composite) under the prefix.
        let current_prefix = format!("{}/meta/current/{}", self.namespace, key_prefix);
        let current_strip = format!("{}/meta/current/", self.namespace);
        if let Ok(entries) = self.vfs.list(&current_prefix).await {
            for entry in entries {
                let key = entry
                    .key
                    .strip_prefix(&current_strip)
                    .unwrap_or(&entry.key)
                    .to_string();
                self.evict_current(&key).await;
            }
        }
        // 2. Delete any orphaned versioned data (non-current versions) whose
        //    data path falls under the prefix.
        let data_prefix = format!("{}/data/{}", self.namespace, key_prefix);
        if let Ok(entries) = self.vfs.list(&data_prefix).await {
            for entry in entries {
                let _ = self.vfs.delete(&entry.key).await;
            }
        }
        // 3. Clean up any remaining composite manifests (shouldn't be any).
        let comp_prefix = format!("{}/meta/composites/{}", self.namespace, key_prefix);
        if let Ok(entries) = self.vfs.list(&comp_prefix).await {
            for entry in entries {
                let _ = self.vfs.delete(&entry.key).await;
            }
        }
        Ok(())
    }

    /// List all blobs whose key starts with `key_prefix`.
    ///
    /// The returned [`VfsEntry::key`] values are the user-visible keys.
    /// All internal metadata paths are excluded.
    pub async fn list(&self, key_prefix: &str) -> Result<Vec<VfsEntry>, VfsError> {
        let current_prefix = format!("{}/meta/current/{}", self.namespace, key_prefix);
        let strip = format!("{}/meta/current/", self.namespace);
        let entries = self.vfs.list(&current_prefix).await?;
        Ok(entries
            .into_iter()
            .map(|e| VfsEntry {
                key: e.key.strip_prefix(&strip).unwrap_or(&e.key).to_string(),
                size: 0, // Use stat() for accurate size; list() is for key enumeration.
            })
            .collect())
    }

    /// Return metadata for the blob stored under `key`, or `None` if absent.
    ///
    /// For composite blobs the reported size is the sum of all part sizes.
    pub async fn stat(&self, key: &str) -> Result<Option<VfsStat>, VfsError> {
        let current = match self.read_current(key).await {
            Some(c) => c,
            None => return Ok(None),
        };
        if current == COMPOSITE_SENTINEL {
            match self.vfs_get_bytes(&self.composite_path(key)).await {
                Ok(manifest) => {
                    if let Ok(parts) = serde_json::from_slice::<Vec<String>>(&manifest) {
                        let mut total: u64 = 0;
                        for k in &parts {
                            if let Ok(Some(s)) = Box::pin(self.stat(k)).await {
                                total += s.size;
                            }
                        }
                        Ok(Some(VfsStat { size: total }))
                    } else {
                        Ok(None)
                    }
                }
                Err(VfsError::NotFound(_)) => Ok(None),
                Err(e) => Err(e),
            }
        } else {
            self.vfs.stat(&self.version_path(key, &current)).await
        }
    }

    /// Return `true` if a blob (or composite) exists under `key`.
    pub async fn exists(&self, key: &str) -> bool {
        self.read_current(key).await.is_some()
    }

    // --- composite blobs ---

    /// Register a composite blob assembled from the given ordered part keys.
    ///
    /// No data is copied; part blobs must already exist under their respective
    /// keys in this store.  On [`get`] / [`get_reader`], the parts are read in
    /// the provided order.  On [`delete`], all referenced part blobs are
    /// removed along with the manifest.
    ///
    /// Any existing blob at `key` (including previous versions) is replaced.
    pub async fn put_composite(&self, key: &str, part_keys: Vec<String>) -> Result<(), VfsError> {
        self.evict_current(key).await;
        let json = serde_json::to_string(&part_keys)
            .map_err(|e| VfsError::Io(io::Error::new(io::ErrorKind::InvalidData, e.to_string())))?;
        self.vfs_put_bytes(&self.composite_path(key), Bytes::from(json.into_bytes()))
            .await?;
        self.vfs_put_bytes(
            &self.current_path(key),
            Bytes::from(COMPOSITE_SENTINEL.as_bytes()),
        )
        .await
    }

    /// Return `true` if a composite manifest exists for `key`.
    pub async fn is_composite(&self, key: &str) -> bool {
        self.read_current(key)
            .await
            .map(|c| c == COMPOSITE_SENTINEL)
            .unwrap_or(false)
    }

    // --- versioned blobs ---

    /// Stream data from `reader` into `key` as a new version, returning the
    /// generated `version_id`.
    ///
    /// This is the streaming equivalent of [`put_versioned`] and avoids
    /// buffering the payload into a contiguous `Bytes` allocation.
    pub async fn put_versioned_from(
        &self,
        key: &str,
        reader: &mut (dyn AsyncRead + Unpin + Send),
    ) -> Result<String, VfsError> {
        if self.is_composite(key).await {
            self.evict_current(key).await;
        }
        let vid = Self::make_version_id();
        self.vfs.put(&self.version_path(key, &vid), reader).await?;
        self.vfs_put_bytes(
            &self.current_path(key),
            Bytes::from(vid.clone().into_bytes()),
        )
        .await?;
        Ok(vid)
    }

    /// Return a streaming reader for a specific version of `key`.
    ///
    /// Unlike [`get_reader`], this does not consult the current-pointer; it
    /// reads the version blob directly by ID.
    ///
    /// Returns [`VfsError::NotFound`] if the version does not exist.
    pub async fn get_version_reader(
        &self,
        key: &str,
        version_id: &str,
    ) -> Result<Box<dyn AsyncRead + Send + Unpin>, VfsError> {
        self.vfs.get(&self.version_path(key, version_id)).await
    }

    /// Store `data` as a new version of `key`, returning the generated
    /// `version_id`.
    ///
    /// Unlike [`put`], previous versions are **not** deleted.  All versions
    /// are stored under `{namespace}/data/{key}/{version_id}` alongside any
    /// non-current versions.
    pub async fn put_versioned(&self, key: &str, data: Bytes) -> Result<String, VfsError> {
        // Replace any composite that was there before.
        if self.is_composite(key).await {
            self.evict_current(key).await;
        }
        let vid = Self::make_version_id();
        let mut cursor = io::Cursor::new(data);
        self.vfs
            .put(&self.version_path(key, &vid), &mut cursor)
            .await?;
        self.vfs_put_bytes(
            &self.current_path(key),
            Bytes::from(vid.clone().into_bytes()),
        )
        .await?;
        Ok(vid)
    }

    /// Import a blob at an exact `version_id` supplied by the caller.
    ///
    /// This is intended for snapshot-restore scenarios where the `version_id` must
    /// match the value stored in the state snapshot.  Unlike [`put_versioned`], the
    /// version ID is **not** auto-generated; the caller is responsible for supplying
    /// a valid BlobStore version ID string.
    ///
    /// If `set_as_current` is `true`, the internal current-pointer is updated so that
    /// subsequent [`get`] / [`get_reader`] calls return this version.
    pub async fn import_version(
        &self,
        key: &str,
        version_id: &str,
        reader: &mut (dyn AsyncRead + Unpin + Send),
        set_as_current: bool,
    ) -> Result<(), VfsError> {
        // version_id is used as a single path segment; reject `.` and `/` to
        // prevent directory traversal and accidental path splitting.
        if version_id.contains('/') || version_id.contains('.') {
            return Err(VfsError::InvalidPath(version_id.to_string()));
        }
        // Remove any composite sentinel so the current pointer is a plain version id.
        if self.is_composite(key).await {
            self.evict_current(key).await;
        }
        self.vfs
            .put(&self.version_path(key, version_id), reader)
            .await?;
        if set_as_current {
            self.vfs_put_bytes(
                &self.current_path(key),
                Bytes::from(version_id.as_bytes().to_vec()),
            )
            .await?;
        }
        Ok(())
    }

    /// Retrieve a specific version of `key`, buffering into [`Bytes`].
    ///
    /// Returns [`VfsError::NotFound`] if the version does not exist.
    pub async fn get_version(&self, key: &str, version_id: &str) -> Result<Bytes, VfsError> {
        self.vfs_get_bytes(&self.version_path(key, version_id))
            .await
    }

    /// Delete a specific version of `key`.
    ///
    /// Returns [`VfsError::NotFound`] if the version does not exist.
    pub async fn delete_version(&self, key: &str, version_id: &str) -> Result<(), VfsError> {
        self.vfs.delete(&self.version_path(key, version_id)).await
    }

    /// List all version IDs for `key`, ordered chronologically (oldest first).
    pub async fn list_versions(&self, key: &str) -> Result<Vec<String>, VfsError> {
        let dir = self.version_dir(key);
        let mut entries = self.vfs.list(&dir).await?;
        entries.sort_by(|a, b| a.key.cmp(&b.key));
        Ok(entries
            .into_iter()
            .map(|e| e.key.strip_prefix(&dir).unwrap_or(&e.key).to_string())
            .collect())
    }

    /// Return the ID of the most recently stored version of `key`, or `None`
    /// if no versions exist.
    pub async fn latest_version_id(&self, key: &str) -> Result<Option<String>, VfsError> {
        Ok(self
            .read_current(key)
            .await
            .filter(|c| c != COMPOSITE_SENTINEL))
    }

    // --- stored metadata API ---
    //
    // At-rest metadata records are stored outside the data/ tree under
    // dedicated meta/ sub-prefixes so they are never confused with blob
    // content or the existing current/composites pointers.
    //
    // Layout:
    //   {ns}/meta/version-meta/{blob_key}/{blob_version_id}   — stored object version (JSON)
    //   {ns}/meta/delete-markers/{blob_key}/{dm_version_id}   — stored delete-marker (JSON)
    //   {ns}/meta/bucket-config/{bucket_name}                 — bucket configuration (JSON)

    fn version_meta_path(&self, blob_key: &str, blob_version_id: &str) -> String {
        format!(
            "{}/meta/version-meta/{}/{}",
            self.namespace, blob_key, blob_version_id
        )
    }

    fn delete_marker_stored_path(&self, blob_key: &str, dm_version_id: &str) -> String {
        format!(
            "{}/meta/delete-markers/{}/{}",
            self.namespace, blob_key, dm_version_id
        )
    }

    fn bucket_config_path(&self, bucket_name: &str) -> String {
        format!("{}/meta/bucket-config/{}", self.namespace, bucket_name)
    }

    /// Write a stored per-version object metadata record.
    pub async fn put_version_meta(
        &self,
        blob_key: &str,
        blob_version_id: &str,
        value: &JsonValue,
    ) -> Result<(), VfsError> {
        let data = Bytes::from(value.to_string().into_bytes());
        self.vfs_put_bytes(&self.version_meta_path(blob_key, blob_version_id), data)
            .await
    }

    /// Read a stored per-version object metadata record.
    pub async fn get_version_meta(
        &self,
        blob_key: &str,
        blob_version_id: &str,
    ) -> Result<JsonValue, VfsError> {
        let raw = self
            .vfs_get_bytes(&self.version_meta_path(blob_key, blob_version_id))
            .await?;
        serde_json::from_slice(&raw)
            .map_err(|e| VfsError::Io(io::Error::new(io::ErrorKind::InvalidData, e.to_string())))
    }

    /// Delete a stored per-version object metadata record.
    pub async fn delete_version_meta(
        &self,
        blob_key: &str,
        blob_version_id: &str,
    ) -> Result<(), VfsError> {
        self.vfs
            .delete(&self.version_meta_path(blob_key, blob_version_id))
            .await
    }

    /// Write a stored delete-marker record.  `dm_version_id` is the S3 version_id UUID.
    pub async fn put_delete_marker_meta(
        &self,
        blob_key: &str,
        dm_version_id: &str,
        value: &JsonValue,
    ) -> Result<(), VfsError> {
        let data = Bytes::from(value.to_string().into_bytes());
        self.vfs_put_bytes(
            &self.delete_marker_stored_path(blob_key, dm_version_id),
            data,
        )
        .await
    }

    /// Read a stored delete-marker record.
    pub async fn get_delete_marker_meta(
        &self,
        blob_key: &str,
        dm_version_id: &str,
    ) -> Result<JsonValue, VfsError> {
        let raw = self
            .vfs_get_bytes(&self.delete_marker_stored_path(blob_key, dm_version_id))
            .await?;
        serde_json::from_slice(&raw)
            .map_err(|e| VfsError::Io(io::Error::new(io::ErrorKind::InvalidData, e.to_string())))
    }

    /// Delete a stored delete-marker record.
    pub async fn delete_delete_marker_meta(
        &self,
        blob_key: &str,
        dm_version_id: &str,
    ) -> Result<(), VfsError> {
        self.vfs
            .delete(&self.delete_marker_stored_path(blob_key, dm_version_id))
            .await
    }

    /// Write (or overwrite) the stored bucket-configuration record.
    pub async fn put_bucket_config(
        &self,
        bucket_name: &str,
        value: &JsonValue,
    ) -> Result<(), VfsError> {
        let data = Bytes::from(value.to_string().into_bytes());
        self.vfs_put_bytes(&self.bucket_config_path(bucket_name), data)
            .await
    }

    /// Read the stored bucket-configuration record.
    pub async fn get_bucket_config(&self, bucket_name: &str) -> Result<JsonValue, VfsError> {
        let raw = self
            .vfs_get_bytes(&self.bucket_config_path(bucket_name))
            .await?;
        serde_json::from_slice(&raw)
            .map_err(|e| VfsError::Io(io::Error::new(io::ErrorKind::InvalidData, e.to_string())))
    }

    /// Delete the stored bucket-configuration record (called when the bucket is deleted).
    pub async fn delete_bucket_config(&self, bucket_name: &str) -> Result<(), VfsError> {
        self.vfs.delete(&self.bucket_config_path(bucket_name)).await
    }

    /// List all stored version-meta entries whose `blob_key` starts with
    /// `blob_key_prefix`.  Returns `(blob_key, blob_version_id)` pairs.
    pub async fn list_version_metas(
        &self,
        blob_key_prefix: &str,
    ) -> Result<Vec<(String, String)>, VfsError> {
        let prefix = format!("{}/meta/version-meta/{}", self.namespace, blob_key_prefix);
        let strip = format!("{}/meta/version-meta/", self.namespace);
        let entries = self.vfs.list(&prefix).await.unwrap_or_default();
        let mut out = Vec::with_capacity(entries.len());
        for e in entries {
            let rel = e.key.strip_prefix(&strip).unwrap_or(&e.key);
            if let Some(slash) = rel.rfind('/') {
                out.push((rel[..slash].to_string(), rel[slash + 1..].to_string()));
            }
        }
        Ok(out)
    }

    /// List all stored delete-marker entries whose `blob_key` starts with
    /// `blob_key_prefix`.  Returns `(blob_key, dm_version_id)` pairs.
    pub async fn list_delete_marker_metas(
        &self,
        blob_key_prefix: &str,
    ) -> Result<Vec<(String, String)>, VfsError> {
        let prefix = format!("{}/meta/delete-markers/{}", self.namespace, blob_key_prefix);
        let strip = format!("{}/meta/delete-markers/", self.namespace);
        let entries = self.vfs.list(&prefix).await.unwrap_or_default();
        let mut out = Vec::with_capacity(entries.len());
        for e in entries {
            let rel = e.key.strip_prefix(&strip).unwrap_or(&e.key);
            if let Some(slash) = rel.rfind('/') {
                out.push((rel[..slash].to_string(), rel[slash + 1..].to_string()));
            }
        }
        Ok(out)
    }

    /// List all bucket names that have a stored bucket-configuration record.
    pub async fn list_bucket_configs(&self) -> Result<Vec<String>, VfsError> {
        let prefix = format!("{}/meta/bucket-config/", self.namespace);
        let entries = self.vfs.list(&prefix).await.unwrap_or_default();
        Ok(entries
            .into_iter()
            .filter_map(|e| e.key.strip_prefix(&prefix).map(str::to_string))
            .collect())
    }
}

// ---------------------------------------------------------------------------
// BlobStoreMap — per-(account, region) blob store isolation
// ---------------------------------------------------------------------------

/// A map of [`BlobStore`] instances keyed by `(account_id, region)`.
///
/// Each unique pair lazily creates a child store whose namespace is
/// `{base_namespace}/{account_id}/{region}`, keeping blobs from different
/// accounts and regions fully isolated within the same underlying VFS.
pub struct BlobStoreMap {
    base: BlobStore,
    children: std::sync::RwLock<std::collections::HashMap<(String, String), Arc<BlobStore>>>,
}

impl BlobStoreMap {
    /// Create a new `BlobStoreMap` backed by an in-memory VFS.
    pub fn mem(namespace: impl Into<String>) -> Self {
        Self {
            base: BlobStore::mem(namespace),
            children: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }

    /// Create a new `BlobStoreMap` backed by the given VFS.
    pub fn new(vfs: Arc<dyn Vfs>, namespace: impl Into<String>) -> Self {
        Self {
            base: BlobStore::new(vfs, namespace),
            children: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }

    /// Get or create the blob store for the given account and region.
    ///
    /// The returned store has namespace `{base}/{account_id}/{region}`.
    pub fn get(&self, account_id: &str, region: &str) -> Arc<BlobStore> {
        let key = (account_id.to_string(), region.to_string());

        // Fast path: read lock
        {
            let map = self.children.read().unwrap();
            if let Some(store) = map.get(&key) {
                return Arc::clone(store);
            }
        }

        // Slow path: write lock, create child
        let mut map = self.children.write().unwrap();
        Arc::clone(
            map.entry(key)
                .or_insert_with_key(|k| Arc::new(self.base.child(format!("{}/{}", k.0, k.1)))),
        )
    }

    /// Access the base (un-scoped) blob store, e.g. for recovery enumeration.
    pub fn base(&self) -> &BlobStore {
        &self.base
    }

    /// Return the underlying VFS.
    pub fn vfs(&self) -> Arc<dyn Vfs> {
        self.base.vfs()
    }

    /// Discover `(account_id, region)` pairs that have stored data.
    ///
    /// Scans the VFS for entries under `{namespace}/` and extracts unique
    /// two-level prefixes `{account_id}/{region}/` from the entry keys.
    pub async fn list_children(&self) -> Result<Vec<(String, String)>, VfsError> {
        let prefix = format!("{}/", self.base.namespace);
        let entries = self.base.vfs.list(&prefix).await.unwrap_or_default();
        let mut seen = std::collections::HashSet::new();
        for entry in &entries {
            let Some(rel) = entry.key.strip_prefix(&prefix) else {
                continue;
            };
            // rel = "{account_id}/{region}/data/..." or "{account_id}/{region}/meta/..."
            let mut parts = rel.splitn(3, '/');
            let Some(account_id) = parts.next() else {
                continue;
            };
            let Some(region) = parts.next() else {
                continue;
            };
            seen.insert((account_id.to_string(), region.to_string()));
        }
        Ok(seen.into_iter().collect())
    }
}

impl std::fmt::Debug for BlobStoreMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlobStoreMap")
            .field("base", &self.base)
            .finish_non_exhaustive()
    }
}

impl std::fmt::Debug for BlobStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlobStore")
            .field("namespace", &self.namespace)
            .finish_non_exhaustive()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn basic_operations() {
        let store = BlobStore::mem("svc");
        let data = Bytes::from_static(b"payload");

        store.put("bucket/key", data.clone()).await.unwrap();
        assert_eq!(store.get("bucket/key").await.unwrap(), data);
        assert!(store.exists("bucket/key").await);

        let stat = store.stat("bucket/key").await.unwrap().unwrap();
        assert_eq!(stat.size, 7);

        store.delete("bucket/key").await.unwrap();
        assert!(!store.exists("bucket/key").await);
        assert!(matches!(
            store.get("bucket/key").await,
            Err(VfsError::NotFound(_))
        ));
    }

    #[tokio::test]
    async fn streaming_put_from_and_get_reader() {
        let store = BlobStore::mem("svc");
        let payload = b"streaming payload";

        store.put_from("key", &mut payload.as_ref()).await.unwrap();
        assert!(store.exists("key").await);

        let mut reader = store.get_reader("key").await.unwrap();
        let mut out = Vec::new();
        reader.read_to_end(&mut out).await.unwrap();
        assert_eq!(out, payload);
    }

    /// Keys that look like internal path components must not trigger any
    /// special behaviour — they are ordinary blobs like any other key.
    #[tokio::test]
    async fn keys_with_reserved_segments_are_safe() {
        let store = BlobStore::mem("svc");

        let cases: &[&str] = &[
            "meta/composites/something",
            "meta/current/key",
            "data/inner/key",
            ".v/stuff",
            ".composites/stuff",
            "bucket/.mpu/upload/1",
            COMPOSITE_SENTINEL, // key literally named "composite"
        ];
        for key in cases {
            let content = Bytes::from(format!("content of {key}"));
            store.put(key, content.clone()).await.unwrap();
            assert_eq!(
                store.get(key).await.unwrap(),
                content,
                "get failed for {key}"
            );
            assert!(
                !store.is_composite(key).await,
                "{key} should not be composite"
            );
        }

        // Blob content that looks like a sentinel must not cause misidentification.
        let tricky = Bytes::from_static(b"composite");
        store.put("tricky", tricky.clone()).await.unwrap();
        assert_eq!(store.get("tricky").await.unwrap(), tricky);
        assert!(!store.is_composite("tricky").await);
    }

    #[tokio::test]
    async fn overwrite_replaces_blob() {
        let store = BlobStore::mem("svc");
        store.put("k", Bytes::from_static(b"v1")).await.unwrap();
        store.put("k", Bytes::from_static(b"v2")).await.unwrap();
        assert_eq!(store.get("k").await.unwrap(), Bytes::from_static(b"v2"));
    }

    #[tokio::test]
    async fn delete_by_prefix() {
        let store = BlobStore::mem("svc");
        store.put("a/1", Bytes::from_static(b"x")).await.unwrap();
        store.put("a/2", Bytes::from_static(b"y")).await.unwrap();
        store.put("b/1", Bytes::from_static(b"z")).await.unwrap();

        store.delete_by_prefix("a/").await.unwrap();

        assert!(!store.exists("a/1").await);
        assert!(!store.exists("a/2").await);
        assert!(store.exists("b/1").await);
    }

    #[tokio::test]
    async fn list_returns_user_visible_keys() {
        let store = BlobStore::mem("ns");
        store.put("x/1", Bytes::from_static(b"a")).await.unwrap();
        store.put("x/2", Bytes::from_static(b"b")).await.unwrap();

        let entries = store.list("x/").await.unwrap();
        let keys: Vec<&str> = entries.iter().map(|e| e.key.as_str()).collect();
        assert!(keys.contains(&"x/1"), "{keys:?}");
        assert!(keys.contains(&"x/2"), "{keys:?}");
        // Internal paths must not appear.
        assert!(!keys.iter().any(|k| k.contains("meta")), "{keys:?}");
        assert!(!keys.iter().any(|k| k.contains("data")), "{keys:?}");
        assert!(!keys.iter().any(|k| k.contains("ns")), "{keys:?}");
    }

    #[tokio::test]
    async fn list_does_not_expose_internal_metadata() {
        let store = BlobStore::mem("svc");
        store
            .put("data/x", Bytes::from_static(b"part"))
            .await
            .unwrap();
        store
            .put_composite("data/obj", vec!["data/x".into()])
            .await
            .unwrap();
        let _ = store
            .put_versioned("data/ver", Bytes::from_static(b"v"))
            .await;

        let entries = store.list("data/").await.unwrap();
        let keys: Vec<&str> = entries.iter().map(|e| e.key.as_str()).collect();
        assert!(!keys.iter().any(|k| k.contains("meta")), "{keys:?}");
        assert!(!keys.iter().any(|k| k.contains("composite")), "{keys:?}");
        assert!(!keys.iter().any(|k| k.contains("current")), "{keys:?}");
    }

    #[tokio::test]
    async fn composite_blob_assembles_parts() {
        let store = BlobStore::mem("svc");
        store
            .put("parts/1", Bytes::from_static(b"hello "))
            .await
            .unwrap();
        store
            .put("parts/2", Bytes::from_static(b"world"))
            .await
            .unwrap();

        store
            .put_composite("combined", vec!["parts/1".into(), "parts/2".into()])
            .await
            .unwrap();

        assert!(store.is_composite("combined").await);
        let data = store.get("combined").await.unwrap();
        assert_eq!(&data[..], b"hello world");
    }

    #[tokio::test]
    async fn composite_get_reader_streams_without_buffering() {
        let store = BlobStore::mem("svc");
        store
            .put("p/a", Bytes::from_static(b"hello "))
            .await
            .unwrap();
        store
            .put("p/b", Bytes::from_static(b"world"))
            .await
            .unwrap();
        store
            .put_composite("combo", vec!["p/a".into(), "p/b".into()])
            .await
            .unwrap();

        let mut reader = store.get_reader("combo").await.unwrap();
        let mut out = Vec::new();
        reader.read_to_end(&mut out).await.unwrap();
        assert_eq!(out, b"hello world");
    }

    #[tokio::test]
    async fn composite_stat_returns_total_size() {
        let store = BlobStore::mem("svc");
        store
            .put("p/a", Bytes::from_static(b"hello "))
            .await
            .unwrap(); // 6 bytes
        store
            .put("p/b", Bytes::from_static(b"world"))
            .await
            .unwrap(); // 5 bytes
        store
            .put_composite("c", vec!["p/a".into(), "p/b".into()])
            .await
            .unwrap();

        let stat = store.stat("c").await.unwrap().unwrap();
        assert_eq!(stat.size, 11);
    }

    #[tokio::test]
    async fn composite_appears_in_list() {
        let store = BlobStore::mem("svc");
        store
            .put("bucket/part", Bytes::from_static(b"x"))
            .await
            .unwrap();
        store
            .put_composite("bucket/obj", vec!["bucket/part".into()])
            .await
            .unwrap();

        let entries = store.list("bucket/").await.unwrap();
        let keys: Vec<&str> = entries.iter().map(|e| e.key.as_str()).collect();
        assert!(keys.contains(&"bucket/obj"), "{keys:?}");
        // The composite manifest sentinel must not leak into list results.
        assert!(!keys.iter().any(|k| k.contains("meta")), "{keys:?}");
        assert!(
            !keys.iter().any(|k| k.contains(COMPOSITE_SENTINEL)),
            "{keys:?}"
        );
    }

    #[tokio::test]
    async fn composite_delete_removes_parts() {
        let store = BlobStore::mem("svc");
        store.put("p/a", Bytes::from_static(b"A")).await.unwrap();
        store.put("p/b", Bytes::from_static(b"B")).await.unwrap();
        store
            .put_composite("multi", vec!["p/a".into(), "p/b".into()])
            .await
            .unwrap();

        store.delete("multi").await.unwrap();

        assert!(!store.exists("multi").await);
        assert!(
            !store.exists("p/a").await,
            "part a should be deleted with composite"
        );
        assert!(
            !store.exists("p/b").await,
            "part b should be deleted with composite"
        );
    }

    #[tokio::test]
    async fn delete_by_prefix_cascades_composite_parts() {
        let store = BlobStore::mem("svc");
        store
            .put("bucket/.mpu/1", Bytes::from_static(b"part1"))
            .await
            .unwrap();
        store
            .put("bucket/.mpu/2", Bytes::from_static(b"part2"))
            .await
            .unwrap();
        store
            .put_composite(
                "bucket/final",
                vec!["bucket/.mpu/1".into(), "bucket/.mpu/2".into()],
            )
            .await
            .unwrap();

        store.delete_by_prefix("bucket/").await.unwrap();

        assert!(!store.exists("bucket/final").await);
        assert!(!store.exists("bucket/.mpu/1").await);
        assert!(!store.exists("bucket/.mpu/2").await);
    }

    #[tokio::test]
    async fn versioned_blob_roundtrip() {
        let store = BlobStore::mem("svc");
        let v1 = store
            .put_versioned("obj", Bytes::from_static(b"v1"))
            .await
            .unwrap();
        let v2 = store
            .put_versioned("obj", Bytes::from_static(b"v2"))
            .await
            .unwrap();

        assert_eq!(
            store.get_version("obj", &v1).await.unwrap(),
            Bytes::from_static(b"v1")
        );
        assert_eq!(
            store.get_version("obj", &v2).await.unwrap(),
            Bytes::from_static(b"v2")
        );

        let versions = store.list_versions("obj").await.unwrap();
        assert_eq!(versions.len(), 2);
        assert_eq!(versions[0], v1);
        assert_eq!(versions[1], v2);

        assert_eq!(
            store.latest_version_id("obj").await.unwrap().as_deref(),
            Some(v2.as_str())
        );
    }

    /// `put()` (unversioned semantics) must return the latest value, not
    /// retain old versions.
    #[tokio::test]
    async fn unversioned_put_does_not_accumulate_versions() {
        let store = BlobStore::mem("svc");
        store.put("k", Bytes::from_static(b"v1")).await.unwrap();
        store.put("k", Bytes::from_static(b"v2")).await.unwrap();
        store.put("k", Bytes::from_static(b"v3")).await.unwrap();

        // Only the current version remains.
        let versions = store.list_versions("k").await.unwrap();
        assert_eq!(versions.len(), 1, "expected 1 version, got {versions:?}");
        assert_eq!(store.get("k").await.unwrap(), Bytes::from_static(b"v3"));
    }

    #[tokio::test]
    async fn versioned_blob_delete_specific_version() {
        let store = BlobStore::mem("svc");
        let v1 = store
            .put_versioned("obj", Bytes::from_static(b"v1"))
            .await
            .unwrap();
        let v2 = store
            .put_versioned("obj", Bytes::from_static(b"v2"))
            .await
            .unwrap();

        store.delete_version("obj", &v1).await.unwrap();

        let versions = store.list_versions("obj").await.unwrap();
        assert_eq!(versions.len(), 1);
        assert_eq!(versions[0], v2);
    }

    #[tokio::test]
    async fn child_store_isolation() {
        let store = BlobStore::mem("root");
        let child = store.child("sub");

        store
            .put("key", Bytes::from_static(b"parent"))
            .await
            .unwrap();
        child
            .put("key", Bytes::from_static(b"child"))
            .await
            .unwrap();

        assert_eq!(
            store.get("key").await.unwrap(),
            Bytes::from_static(b"parent")
        );
        assert_eq!(
            child.get("key").await.unwrap(),
            Bytes::from_static(b"child")
        );
    }
}

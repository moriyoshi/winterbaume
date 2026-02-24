//! Virtual filesystem abstraction for blob storage.
//!
//! The [`Vfs`] trait abstracts over storage backends so that services can keep
//! large binary blobs out of their primary in-memory state.  Two built-in
//! implementations are provided:
//!
//! * [`MemVfs`] — stores blobs in a `HashMap` protected by an `RwLock`.
//!   This is the default and requires no external resources.
//! * [`FsVfs`] — stores each blob as a file under a base directory on the
//!   local filesystem, enabling off-heap (and cross-restart) persistence.
//!
//! # Async streaming I/O
//!
//! Every data-transfer method on [`Vfs`] is async.  [`put`] accepts a
//! `&mut (dyn AsyncRead + Unpin + Send)` so callers can stream data directly
//! from network buffers without staging the full payload in memory.  [`get`]
//! returns a `Box<dyn AsyncRead + Send + Unpin>` — for [`FsVfs`] this is an
//! open [`tokio::fs::File`] that lets the caller read incrementally; for
//! [`MemVfs`] it is a `Cursor` over the in-memory buffer.
//!
//! The trait uses the `Pin<Box<dyn Future>>` pattern (the same as
//! [`crate::MockService::handle`]) so that `Arc<dyn Vfs>` remains object-safe
//! without requiring the `async-trait` crate.

use std::collections::HashMap;
use std::fmt;
use std::future::Future;
use std::io;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::RwLock;

use bytes::Bytes;
use tokio::io::{AsyncRead, AsyncReadExt};

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// Errors returned by [`Vfs`] operations.
#[derive(Debug)]
pub enum VfsError {
    /// The requested path was not found.
    NotFound(String),
    /// An underlying I/O error (only possible with [`FsVfs`]).
    Io(io::Error),
    /// The path contains invalid components (e.g. `..`) that would escape the
    /// storage root.
    InvalidPath(String),
}

impl fmt::Display for VfsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VfsError::NotFound(p) => write!(f, "blob not found: {p}"),
            VfsError::Io(e) => write!(f, "vfs I/O error: {e}"),
            VfsError::InvalidPath(p) => write!(f, "invalid blob path: {p}"),
        }
    }
}

impl std::error::Error for VfsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            VfsError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for VfsError {
    fn from(e: io::Error) -> Self {
        VfsError::Io(e)
    }
}

// ---------------------------------------------------------------------------
// Supporting types
// ---------------------------------------------------------------------------

/// Metadata about a stored blob.
#[derive(Debug, Clone)]
pub struct VfsStat {
    /// Size in bytes.
    pub size: u64,
}

/// An entry returned by [`Vfs::list`].
#[derive(Debug, Clone)]
pub struct VfsEntry {
    /// The full key of this blob (relative to the store's namespace).
    pub key: String,
    /// Size in bytes.
    pub size: u64,
}

// ---------------------------------------------------------------------------
// Trait
// ---------------------------------------------------------------------------

/// A simple async key-value blob store.
///
/// All paths/keys are `/`-separated strings (similar to URL paths).
/// Implementations must be `Send + Sync` so they can be shared across
/// threads via `Arc<dyn Vfs>`.
///
/// The trait uses explicit `Pin<Box<dyn Future<...>>>` return types so that
/// `dyn Vfs` remains object-safe.
pub trait Vfs: Send + Sync {
    /// Stream `data` into `path`, overwriting any existing entry.
    ///
    /// Returns the number of bytes written.
    fn put<'a>(
        &'a self,
        path: &'a str,
        data: &'a mut (dyn AsyncRead + Unpin + Send),
    ) -> Pin<Box<dyn Future<Output = Result<u64, VfsError>> + Send + 'a>>;

    /// Return a streaming reader for the blob stored at `path`.
    ///
    /// Returns [`VfsError::NotFound`] if the path does not exist.
    fn get<'a>(
        &'a self,
        path: &'a str,
    ) -> Pin<
        Box<dyn Future<Output = Result<Box<dyn AsyncRead + Send + Unpin>, VfsError>> + Send + 'a>,
    >;

    /// Delete the blob at `path`.
    ///
    /// Returns [`VfsError::NotFound`] if the path does not exist.
    fn delete<'a>(
        &'a self,
        path: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<(), VfsError>> + Send + 'a>>;

    /// List all entries whose key starts with `prefix`.
    ///
    /// The prefix may be empty to list everything.
    fn list<'a>(
        &'a self,
        prefix: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<VfsEntry>, VfsError>> + Send + 'a>>;

    /// Return metadata for the blob at `path`, or `None` if it does not exist.
    fn stat<'a>(
        &'a self,
        path: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<VfsStat>, VfsError>> + Send + 'a>>;
}

// ---------------------------------------------------------------------------
// Shared path validation
// ---------------------------------------------------------------------------

/// Reject paths that contain `..` components or null bytes.
///
/// Both [`MemVfs`] and [`FsVfs`] call this before any operation so that they
/// behave identically regardless of backend.  Null bytes are rejected because
/// they terminate C strings used by OS syscalls in ways that could silently
/// truncate the intended path.
pub(crate) fn validate_path(path: &str) -> Result<(), VfsError> {
    if path.contains('\0') {
        return Err(VfsError::InvalidPath(path.to_string()));
    }
    let clean = path.trim_start_matches('/');
    for component in Path::new(clean).components() {
        if component == std::path::Component::ParentDir {
            return Err(VfsError::InvalidPath(path.to_string()));
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// MemVfs
// ---------------------------------------------------------------------------

/// In-memory VFS backed by a `HashMap`.
///
/// This is the default backend used when no explicit VFS is configured.
/// All data is stored on the heap and is lost when the service is dropped.
/// [`get`] returns a [`io::Cursor`] over a ref-counted [`Bytes`] clone so no
/// extra copy of stored data is made.
///
/// Path validation (rejection of `..` components and null bytes) matches
/// [`FsVfs`] so both backends enforce the same invariants.
#[derive(Debug, Default)]
pub struct MemVfs {
    data: RwLock<HashMap<String, Bytes>>,
}

impl MemVfs {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Vfs for MemVfs {
    fn put<'a>(
        &'a self,
        path: &'a str,
        data: &'a mut (dyn AsyncRead + Unpin + Send),
    ) -> Pin<Box<dyn Future<Output = Result<u64, VfsError>> + Send + 'a>> {
        if let Err(e) = validate_path(path) {
            return Box::pin(async { Err(e) });
        }
        Box::pin(async move {
            let mut buf = Vec::new();
            let n = data.read_to_end(&mut buf).await.map_err(VfsError::Io)? as u64;
            self.data
                .write()
                .unwrap()
                .insert(path.to_string(), Bytes::from(buf));
            Ok(n)
        })
    }

    fn get<'a>(
        &'a self,
        path: &'a str,
    ) -> Pin<
        Box<dyn Future<Output = Result<Box<dyn AsyncRead + Send + Unpin>, VfsError>> + Send + 'a>,
    > {
        if let Err(e) = validate_path(path) {
            return Box::pin(async { Err(e) });
        }
        Box::pin(async move {
            let data = self
                .data
                .read()
                .unwrap()
                .get(path)
                .cloned()
                .ok_or_else(|| VfsError::NotFound(path.to_string()))?;
            Ok(Box::new(io::Cursor::new(data)) as Box<dyn AsyncRead + Send + Unpin>)
        })
    }

    fn delete<'a>(
        &'a self,
        path: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<(), VfsError>> + Send + 'a>> {
        if let Err(e) = validate_path(path) {
            return Box::pin(async { Err(e) });
        }
        Box::pin(async move {
            if self.data.write().unwrap().remove(path).is_none() {
                return Err(VfsError::NotFound(path.to_string()));
            }
            Ok(())
        })
    }

    fn list<'a>(
        &'a self,
        prefix: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<VfsEntry>, VfsError>> + Send + 'a>> {
        if let Err(e) = validate_path(prefix) {
            return Box::pin(async { Err(e) });
        }
        Box::pin(async move {
            let guard = self.data.read().unwrap();
            let mut entries: Vec<VfsEntry> = guard
                .iter()
                .filter(|(k, _)| k.starts_with(prefix))
                .map(|(k, v)| VfsEntry {
                    key: k.clone(),
                    size: v.len() as u64,
                })
                .collect();
            entries.sort_by(|a, b| a.key.cmp(&b.key));
            Ok(entries)
        })
    }

    fn stat<'a>(
        &'a self,
        path: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<VfsStat>, VfsError>> + Send + 'a>> {
        if let Err(e) = validate_path(path) {
            return Box::pin(async { Err(e) });
        }
        Box::pin(async move {
            Ok(self.data.read().unwrap().get(path).map(|v| VfsStat {
                size: v.len() as u64,
            }))
        })
    }
}

// ---------------------------------------------------------------------------
// FsVfs
// ---------------------------------------------------------------------------

/// Filesystem-backed VFS using async tokio I/O.
///
/// Each blob is stored as a regular file under `base_dir`.  The blob's path
/// (after any BlobStore namespace prefix is stripped) is mapped directly to a
/// filesystem path under `base_dir`, with `/` becoming the OS path separator.
///
/// Parent directories are created automatically on `put`.  Deleting a blob
/// removes its file but leaves any empty parent directories intact.
///
/// [`put`] streams from the caller's reader directly to disk via
/// [`tokio::io::copy`].  [`get`] returns an open [`tokio::fs::File`] so data
/// can be read incrementally without a full in-memory allocation.  Directory
/// listing uses [`tokio::task::spawn_blocking`] since recursive `read_dir`
/// is not yet offered as a simple async utility by tokio.
#[derive(Debug)]
pub struct FsVfs {
    base_dir: PathBuf,
}

impl FsVfs {
    /// Create an `FsVfs` rooted at `base_dir`.
    ///
    /// The directory is created if it does not already exist.
    pub fn new(base_dir: impl AsRef<Path>) -> io::Result<Self> {
        let base_dir = base_dir.as_ref().to_path_buf();
        std::fs::create_dir_all(&base_dir)?;
        Ok(Self { base_dir })
    }

    fn full_path(&self, path: &str) -> Result<PathBuf, VfsError> {
        validate_path(path)?;
        Ok(self.base_dir.join(path.trim_start_matches('/')))
    }
}

impl Vfs for FsVfs {
    fn put<'a>(
        &'a self,
        path: &'a str,
        data: &'a mut (dyn AsyncRead + Unpin + Send),
    ) -> Pin<Box<dyn Future<Output = Result<u64, VfsError>> + Send + 'a>> {
        match self.full_path(path) {
            Err(e) => Box::pin(async { Err(e) }),
            Ok(fp) => Box::pin(async move {
                if let Some(parent) = fp.parent() {
                    tokio::fs::create_dir_all(parent).await?;
                }
                let mut file = tokio::fs::File::create(&fp).await?;
                let n = tokio::io::copy(data, &mut file).await?;
                Ok(n)
            }),
        }
    }

    fn get<'a>(
        &'a self,
        path: &'a str,
    ) -> Pin<
        Box<dyn Future<Output = Result<Box<dyn AsyncRead + Send + Unpin>, VfsError>> + Send + 'a>,
    > {
        match self.full_path(path) {
            Err(e) => Box::pin(async { Err(e) }),
            Ok(fp) => {
                let path = path.to_string();
                Box::pin(async move {
                    match tokio::fs::File::open(&fp).await {
                        Ok(f) => Ok(Box::new(f) as Box<dyn AsyncRead + Send + Unpin>),
                        Err(e) if e.kind() == io::ErrorKind::NotFound => {
                            Err(VfsError::NotFound(path))
                        }
                        Err(e) => Err(VfsError::Io(e)),
                    }
                })
            }
        }
    }

    fn delete<'a>(
        &'a self,
        path: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<(), VfsError>> + Send + 'a>> {
        match self.full_path(path) {
            Err(e) => Box::pin(async { Err(e) }),
            Ok(fp) => {
                let path = path.to_string();
                Box::pin(async move {
                    match tokio::fs::remove_file(&fp).await {
                        Ok(()) => Ok(()),
                        Err(e) if e.kind() == io::ErrorKind::NotFound => {
                            Err(VfsError::NotFound(path))
                        }
                        Err(e) => Err(VfsError::Io(e)),
                    }
                })
            }
        }
    }

    fn list<'a>(
        &'a self,
        prefix: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<VfsEntry>, VfsError>> + Send + 'a>> {
        if let Err(e) = validate_path(prefix) {
            return Box::pin(async { Err(e) });
        }
        let clean_prefix = prefix.trim_start_matches('/');
        // Walk the parent directory (or the prefix itself if it is a directory)
        // and filter entries by string-prefix matching, matching MemVfs semantics.
        let search_dir = if clean_prefix.is_empty() {
            self.base_dir.clone()
        } else {
            let candidate = self.base_dir.join(clean_prefix);
            if candidate.is_dir() {
                candidate
            } else {
                // The prefix may be a partial path component (e.g. "ns/bucket"
                // should match "ns/bucket-a/obj").  Walk from the parent
                // directory so we can check every entry's key against the
                // string prefix.
                candidate.parent().unwrap_or(&self.base_dir).to_path_buf()
            }
        };
        let base_str = self.base_dir.to_string_lossy().into_owned();
        let prefix_owned = clean_prefix.to_string();
        Box::pin(async move {
            tokio::task::spawn_blocking(move || -> Result<Vec<VfsEntry>, VfsError> {
                let mut entries = Vec::new();
                collect_files(&search_dir, &base_str, &mut entries)?;
                // Apply string-prefix filtering to match the Vfs trait contract.
                entries.retain(|e| e.key.starts_with(&prefix_owned));
                entries.sort_by(|a, b| a.key.cmp(&b.key));
                Ok(entries)
            })
            .await
            .map_err(|e| VfsError::Io(io::Error::other(e.to_string())))?
        })
    }

    fn stat<'a>(
        &'a self,
        path: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<VfsStat>, VfsError>> + Send + 'a>> {
        match self.full_path(path) {
            Err(e) => Box::pin(async { Err(e) }),
            Ok(fp) => Box::pin(async move {
                match tokio::fs::metadata(&fp).await {
                    Ok(m) => Ok(Some(VfsStat { size: m.len() })),
                    Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
                    Err(e) => Err(VfsError::Io(e)),
                }
            }),
        }
    }
}

/// Recursively walk `dir`, appending file entries to `out`.
fn collect_files(dir: &Path, base: &str, out: &mut Vec<VfsEntry>) -> Result<(), VfsError> {
    if !dir.exists() {
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let ft = entry.file_type()?;
        let path = entry.path();
        if ft.is_dir() {
            collect_files(&path, base, out)?;
        } else if ft.is_file() {
            let full = path.to_string_lossy();
            let key = if let Some(stripped) = full.strip_prefix(base) {
                stripped.trim_start_matches(['/', '\\']).to_string()
            } else {
                full.into_owned()
            };
            let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
            out.push(VfsEntry { key, size });
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    async fn roundtrip(vfs: &dyn Vfs) {
        let data = b"hello world";

        // put + get
        vfs.put("ns/a/b", &mut data.as_ref()).await.unwrap();
        let mut buf = Vec::new();
        vfs.get("ns/a/b")
            .await
            .unwrap()
            .read_to_end(&mut buf)
            .await
            .unwrap();
        assert_eq!(buf, data);

        // stat
        let stat = vfs.stat("ns/a/b").await.unwrap().unwrap();
        assert_eq!(stat.size, 11);

        // stat non-existent
        assert!(vfs.stat("ns/a/missing").await.unwrap().is_none());

        // list
        vfs.put("ns/a/c", &mut b"other".as_ref()).await.unwrap();
        let entries = vfs.list("ns/a/").await.unwrap();
        let keys: Vec<&str> = entries.iter().map(|e| e.key.as_str()).collect();
        assert!(keys.contains(&"ns/a/b"), "expected ns/a/b in {keys:?}");
        assert!(keys.contains(&"ns/a/c"), "expected ns/a/c in {keys:?}");

        // delete
        vfs.delete("ns/a/b").await.unwrap();
        assert!(matches!(
            vfs.get("ns/a/b").await,
            Err(VfsError::NotFound(_))
        ));

        // delete non-existent
        assert!(matches!(
            vfs.delete("ns/a/missing").await,
            Err(VfsError::NotFound(_))
        ));
    }

    #[tokio::test]
    async fn mem_vfs_roundtrip() {
        let vfs = MemVfs::new();
        roundtrip(&vfs).await;
    }

    #[tokio::test]
    async fn fs_vfs_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let vfs = FsVfs::new(dir.path()).unwrap();
        roundtrip(&vfs).await;
    }

    async fn rejects_path_traversal(vfs: &dyn Vfs) {
        let traversal_paths = ["../escape", "a/../../escape", "a/../../../etc/passwd"];
        for path in &traversal_paths {
            assert!(
                matches!(
                    vfs.put(path, &mut b"x".as_ref()).await,
                    Err(VfsError::InvalidPath(_))
                ),
                "put({path}) should be rejected"
            );
            assert!(
                matches!(vfs.get(path).await, Err(VfsError::InvalidPath(_))),
                "get({path}) should be rejected"
            );
            assert!(
                matches!(vfs.delete(path).await, Err(VfsError::InvalidPath(_))),
                "delete({path}) should be rejected"
            );
            assert!(
                matches!(vfs.list(path).await, Err(VfsError::InvalidPath(_))),
                "list({path}) should be rejected"
            );
            assert!(
                matches!(vfs.stat(path).await, Err(VfsError::InvalidPath(_))),
                "stat({path}) should be rejected"
            );
        }
    }

    #[tokio::test]
    async fn mem_vfs_rejects_path_traversal() {
        let vfs = MemVfs::new();
        rejects_path_traversal(&vfs).await;
    }

    #[tokio::test]
    async fn fs_vfs_rejects_path_traversal() {
        let dir = tempfile::tempdir().unwrap();
        let vfs = FsVfs::new(dir.path()).unwrap();
        rejects_path_traversal(&vfs).await;
    }
}

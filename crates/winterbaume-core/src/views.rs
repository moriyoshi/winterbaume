//! Unified state view API for service backends.
//!
//! The [`StatefulService`] trait provides a standardized way to access and
//! modify any service's internal state through serde-compatible typed views.
//! View types are separate from internal state types to decouple the
//! serialization contract from implementation details.

use std::pin::Pin;
use std::sync::Arc;

use serde::Serialize;
use serde::de::DeserializeOwned;
use tokio::io::AsyncRead;

use crate::service::MockService;
use crate::vfs::VfsError;

/// Error type for state view operations.
#[derive(Debug)]
pub enum StateViewError {
    /// Serialization or deserialization failed.
    Serialize(serde_json::Error),
    /// The view data is invalid or inconsistent.
    Invalid(String),
    /// A blob operation failed during export or import.
    Blob(VfsError),
}

impl std::fmt::Display for StateViewError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateViewError::Serialize(e) => write!(f, "serialization error: {e}"),
            StateViewError::Invalid(msg) => write!(f, "invalid state: {msg}"),
            StateViewError::Blob(e) => write!(f, "blob error: {e}"),
        }
    }
}

impl std::error::Error for StateViewError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            StateViewError::Serialize(e) => Some(e),
            StateViewError::Invalid(_) => None,
            StateViewError::Blob(e) => Some(e),
        }
    }
}

impl From<VfsError> for StateViewError {
    fn from(e: VfsError) -> Self {
        StateViewError::Blob(e)
    }
}

impl From<serde_json::Error> for StateViewError {
    fn from(e: serde_json::Error) -> Self {
        StateViewError::Serialize(e)
    }
}

/// A listener called when a service's state changes.
///
/// Arguments: `account_id`, `region`, and a reference to the new state snapshot.
/// The listener runs synchronously; use `tokio::spawn` inside for async work.
pub type StateChangeListener<V> = Arc<dyn Fn(&str, &str, &V) + Send + Sync>;

/// Manages state-change subscriptions for a service.
///
/// Embed one instance in each service struct that implements [`StatefulService`]
/// and delegate the required `notifier()` method to it.
pub struct StateChangeNotifier<V> {
    listeners: std::sync::RwLock<Vec<StateChangeListener<V>>>,
}

impl<V: Send + Sync> StateChangeNotifier<V> {
    pub fn new() -> Self {
        Self {
            listeners: std::sync::RwLock::new(Vec::new()),
        }
    }

    /// Register a closure to be called on every state change.
    pub fn subscribe(&self, f: impl Fn(&str, &str, &V) + Send + Sync + 'static) {
        self.listeners.write().unwrap().push(Arc::new(f));
    }

    /// Invoke all registered listeners with the given snapshot.
    pub fn notify(&self, account_id: &str, region: &str, view: &V) {
        let listeners = self.listeners.read().unwrap();
        for listener in listeners.iter() {
            listener(account_id, region, view);
        }
    }
}

impl<V: Send + Sync> Default for StateChangeNotifier<V> {
    fn default() -> Self {
        Self::new()
    }
}

/// A service that exposes typed, serde-compatible views of its internal state.
///
/// Each service implements this trait with a `StateView` type that is a
/// serde-friendly representation of its internal state. The view types
/// are separate from the internal types to decouple serialization from
/// implementation.
///
/// # Example
///
/// ```ignore
/// // Take a snapshot of S3 state
/// let view: S3StateView = s3_service.snapshot("123456789012", "us-east-1");
///
/// // Serialize to JSON
/// let json = serde_json::to_string_pretty(&view).unwrap();
///
/// // Restore from a view
/// let view: S3StateView = serde_json::from_str(&json).unwrap();
/// s3_service.restore("123456789012", "us-east-1", view).unwrap();
/// ```
#[allow(async_fn_in_trait)]
pub trait StatefulService: MockService {
    /// Serde-compatible view of this service's per-region state.
    type StateView: Serialize + DeserializeOwned + Send + Sync;

    /// Take a snapshot of the state for the given account/region as a typed view.
    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView;

    /// Restore state for the given account/region from a typed view.
    /// Replaces the existing state entirely.
    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError>;

    /// Merge a partial view into existing state (additive, does not remove
    /// existing resources).
    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError>;

    /// Returns the state-change notifier embedded in this service.
    ///
    /// Implement by returning a reference to a `StateChangeNotifier` field
    /// on the service struct.
    fn notifier(&self) -> &StateChangeNotifier<Self::StateView>;

    /// Take a snapshot of the given account/region and forward it to all
    /// registered listeners. Call this after any successful state mutation.
    ///
    /// The write lock on the state must be released before calling this
    /// method to avoid a deadlock (snapshot acquires a read lock).
    async fn notify_state_changed(&self, account_id: &str, region: &str) {
        let view = self.snapshot(account_id, region).await;
        self.notifier().notify(account_id, region, &view);
    }
}

// ---------------------------------------------------------------------------
// Blob-backed service extension (visitor pattern, minibatch)
// ---------------------------------------------------------------------------

/// Default number of blobs per minibatch in the default `export_blobs` /
/// `import_blobs` implementations.
pub const DEFAULT_BLOB_BATCH_SIZE: usize = 64;

/// A single blob entry within an export batch.
pub struct BlobExportEntry {
    /// Blob key as it appears in the `StateView`.
    pub key: String,
    /// Streaming reader for the blob content.
    pub reader: Box<dyn AsyncRead + Send + Unpin>,
    /// Size in bytes, if known from a stat call.
    pub size: Option<u64>,
}

/// Callback object for receiving exported blobs in minibatches.
///
/// The returned future borrows `&mut self`, so the implementation can
/// accumulate state across calls without `Arc<Mutex<…>>`.  The caller
/// awaits each future before calling `visit` again, so no aliasing occurs.
///
/// This trait is dyn-compatible — use `&mut dyn BlobVisitor` in generic code.
pub trait BlobVisitor: Send {
    fn visit(
        &mut self,
        batch: Vec<BlobExportEntry>,
    ) -> Pin<Box<dyn Future<Output = Result<(), VfsError>> + Send + '_>>;
}

/// Callback object for supplying blob data during import.
///
/// Same borrowing semantics as [`BlobVisitor`]: the future borrows
/// `&mut self` and is awaited before the next call.
pub trait BlobSource: Send {
    fn fetch(
        &mut self,
        key: String,
    ) -> Pin<
        Box<dyn Future<Output = Result<Box<dyn AsyncRead + Send + Unpin>, VfsError>> + Send + '_>,
    >;
}

/// Extension trait for services whose state references blobs in a backing store.
///
/// The backing store is service-wide (not partitioned by account or region).
/// Export and import use [`BlobVisitor`] / [`BlobSource`] callbacks invoked
/// repeatedly in minibatches.
///
/// # Workflow
///
/// **Export**:
/// ```ignore
/// service.snapshot_with_blobs(&mut my_visitor).await?;
/// ```
///
/// **Import**:
/// ```ignore
/// service.restore_with_blobs(view, &mut my_source).await?;
/// ```
#[allow(async_fn_in_trait)]
pub trait BlobBackedService: StatefulService {
    /// Atomically snapshot metadata and export all blobs.
    ///
    /// Captures the `StateView` and streams all backing store blobs
    /// through the visitor in minibatches. Returns the view consistent
    /// with the exported blobs.
    async fn snapshot_with_blobs(
        &self,
        account_id: &str,
        region: &str,
        visitor: &mut dyn BlobVisitor,
    ) -> Result<Self::StateView, StateViewError>;

    /// Import blobs into the service's backing store from a source.
    ///
    /// The source is called for each blob key referenced in the view;
    /// it returns a reader for that blob's content. Call this before
    /// `restore()` so that blob keys resolve correctly.
    async fn restore_with_blobs(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
        source: &mut dyn BlobSource,
    ) -> Result<(), StateViewError>;
}

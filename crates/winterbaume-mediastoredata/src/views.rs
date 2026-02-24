//! Serde-compatible view types for MediaStoreData state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::MediaStoreDataService;
use crate::state::MediaStoreDataState;

/// Serializable view of a single stored object.
///
/// The binary body is excluded from snapshots; only metadata is captured.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MediaStoreObjectView {
    pub path: String,
    pub content_type: String,
    pub content_length: u64,
    pub etag: String,
    pub last_modified: String,
    pub cache_control: Option<String>,
}

/// Serializable view of the entire MediaStoreData state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MediaStoreDataStateView {
    #[serde(default)]
    pub objects: HashMap<String, MediaStoreObjectView>,
}

// --- From internal types to view types ---

impl From<&crate::types::MediaStoreObject> for MediaStoreObjectView {
    fn from(obj: &crate::types::MediaStoreObject) -> Self {
        MediaStoreObjectView {
            path: obj.path.clone(),
            content_type: obj.content_type.clone(),
            content_length: obj.content_length,
            etag: obj.etag.clone(),
            last_modified: obj.last_modified.to_rfc3339(),
            cache_control: obj.cache_control.clone(),
        }
    }
}

impl From<&MediaStoreDataState> for MediaStoreDataStateView {
    fn from(state: &MediaStoreDataState) -> Self {
        MediaStoreDataStateView {
            objects: state
                .objects
                .iter()
                .map(|(k, v)| (k.clone(), MediaStoreObjectView::from(v)))
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<MediaStoreObjectView> for crate::types::MediaStoreObject {
    fn from(v: MediaStoreObjectView) -> Self {
        let last_modified = chrono::DateTime::parse_from_rfc3339(&v.last_modified)
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now());
        crate::types::MediaStoreObject {
            path: v.path.clone(),
            content_type: v.content_type,
            body: bytes::Bytes::new(),
            content_length: v.content_length,
            etag: v.etag,
            last_modified,
            cache_control: v.cache_control,
        }
    }
}

impl From<MediaStoreDataStateView> for MediaStoreDataState {
    fn from(view: MediaStoreDataStateView) -> Self {
        MediaStoreDataState {
            objects: view
                .objects
                .into_iter()
                .map(|(k, v)| (k, crate::types::MediaStoreObject::from(v)))
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for MediaStoreDataService {
    type StateView = MediaStoreDataStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        MediaStoreDataStateView::from(&*guard)
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
            *guard = MediaStoreDataState::from(view);
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
            for (path, obj_view) in view.objects {
                guard
                    .objects
                    .insert(path, crate::types::MediaStoreObject::from(obj_view));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

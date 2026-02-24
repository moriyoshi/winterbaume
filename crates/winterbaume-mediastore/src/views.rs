//! Serde-compatible view types for MediaStore state snapshots.

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::MediaStoreService;
use crate::state::MediaStoreState;

/// Serializable view of a tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagView {
    pub key: String,
    pub value: Option<String>,
}

/// Serializable view of a single MediaStore container.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContainerView {
    pub arn: String,
    pub name: String,
    pub endpoint: String,
    pub status: String,
    pub creation_time: String,
    pub lifecycle_policy: Option<String>,
    pub policy: Option<String>,
    /// MetricPolicy stored as a raw JSON string to avoid serde_json::Value in public fields.
    pub metric_policy_json: Option<String>,
    #[serde(default)]
    pub tags: Vec<TagView>,
}

/// Serializable view of the entire MediaStore state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MediaStoreStateView {
    #[serde(default)]
    pub containers: std::collections::HashMap<String, ContainerView>,
}

// --- From internal types to view types ---

impl From<&crate::types::Container> for ContainerView {
    fn from(c: &crate::types::Container) -> Self {
        ContainerView {
            arn: c.arn.clone(),
            name: c.name.clone(),
            endpoint: c.endpoint.clone(),
            status: c.status.clone(),
            creation_time: c.creation_time.clone(),
            lifecycle_policy: c.lifecycle_policy.clone(),
            policy: c.policy.clone(),
            metric_policy_json: c
                .metric_policy
                .as_ref()
                .and_then(|v| serde_json::to_string(v).ok()),
            tags: c
                .tags
                .as_ref()
                .map(|tags| {
                    tags.iter()
                        .map(|t| TagView {
                            key: t.key.clone(),
                            value: t.value.clone(),
                        })
                        .collect()
                })
                .unwrap_or_default(),
        }
    }
}

impl From<&MediaStoreState> for MediaStoreStateView {
    fn from(state: &MediaStoreState) -> Self {
        MediaStoreStateView {
            containers: state
                .containers
                .iter()
                .map(|(k, v)| (k.clone(), ContainerView::from(v)))
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<ContainerView> for crate::types::Container {
    fn from(v: ContainerView) -> Self {
        let metric_policy = v
            .metric_policy_json
            .as_deref()
            .and_then(|s| serde_json::from_str(s).ok());
        let tags = if v.tags.is_empty() {
            None
        } else {
            Some(
                v.tags
                    .into_iter()
                    .map(|t| crate::types::Tag {
                        key: t.key,
                        value: t.value,
                    })
                    .collect(),
            )
        };
        crate::types::Container {
            arn: v.arn,
            name: v.name,
            endpoint: v.endpoint,
            status: v.status,
            creation_time: v.creation_time,
            lifecycle_policy: v.lifecycle_policy,
            policy: v.policy,
            metric_policy,
            tags,
        }
    }
}

impl From<MediaStoreStateView> for MediaStoreState {
    fn from(view: MediaStoreStateView) -> Self {
        MediaStoreState {
            containers: view
                .containers
                .into_iter()
                .map(|(k, v)| (k, crate::types::Container::from(v)))
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for MediaStoreService {
    type StateView = MediaStoreStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        MediaStoreStateView::from(&*guard)
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
            *guard = MediaStoreState::from(view);
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
            for (name, container_view) in view.containers {
                guard
                    .containers
                    .insert(name, crate::types::Container::from(container_view));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

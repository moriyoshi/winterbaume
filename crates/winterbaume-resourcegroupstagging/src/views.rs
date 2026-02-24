//! Serde-compatible view types for Tagging state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::TaggingService;
use crate::state::TaggingState;
use crate::types::TaggedResource;

/// Serializable view of the entire Tagging state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaggingStateView {
    /// Resources keyed by ARN.
    #[serde(default)]
    pub resources: HashMap<String, TaggedResourceView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaggedResourceView {
    pub arn: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

// --- From internal types to view types ---

impl From<&TaggingState> for TaggingStateView {
    fn from(state: &TaggingState) -> Self {
        TaggingStateView {
            resources: state
                .resources
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        TaggedResourceView {
                            arn: v.arn.clone(),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<TaggingStateView> for TaggingState {
    fn from(view: TaggingStateView) -> Self {
        TaggingState {
            resources: view
                .resources
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        TaggedResource {
                            arn: v.arn,
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for TaggingService {
    type StateView = TaggingStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        TaggingStateView::from(&*guard)
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
            *guard = TaggingState::from(view);
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
            for (k, v) in view.resources {
                guard
                    .resources
                    .entry(k.clone())
                    .or_insert_with(|| TaggedResource {
                        arn: k,
                        tags: HashMap::new(),
                    })
                    .tags
                    .extend(v.tags);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

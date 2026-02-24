//! Serde-compatible view types for DSQL state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::DsqlService;
use crate::state::DsqlState;
use crate::types::Cluster;

/// Serializable view of the entire DSQL state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DsqlStateView {
    /// Clusters keyed by cluster identifier.
    #[serde(default)]
    pub clusters: HashMap<String, ClusterView>,
    /// Maps client_token -> cluster identifier for idempotency.
    #[serde(default)]
    pub client_tokens: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterView {
    pub identifier: String,
    pub arn: String,
    pub status: String,
    pub creation_time: DateTime<Utc>,
    pub deletion_protection_enabled: bool,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

// --- From internal types to view types ---

impl From<&DsqlState> for DsqlStateView {
    fn from(state: &DsqlState) -> Self {
        DsqlStateView {
            clusters: state
                .clusters
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ClusterView {
                            identifier: v.identifier.clone(),
                            arn: v.arn.clone(),
                            status: v.status.clone(),
                            creation_time: v.creation_time,
                            deletion_protection_enabled: v.deletion_protection_enabled,
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            client_tokens: state.client_tokens.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<DsqlStateView> for DsqlState {
    fn from(view: DsqlStateView) -> Self {
        DsqlState {
            clusters: view
                .clusters
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Cluster {
                            identifier: v.identifier,
                            arn: v.arn,
                            status: v.status,
                            creation_time: v.creation_time,
                            deletion_protection_enabled: v.deletion_protection_enabled,
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            client_tokens: view.client_tokens,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for DsqlService {
    type StateView = DsqlStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        DsqlStateView::from(&*guard)
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
            *guard = DsqlState::from(view);
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
            let incoming = DsqlState::from(view);
            for (k, v) in incoming.clusters {
                guard.clusters.insert(k, v);
            }
            for (k, v) in incoming.client_tokens {
                guard.client_tokens.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

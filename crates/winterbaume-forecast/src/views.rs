//! Serde-compatible view types for Forecast state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ForecastService;
use crate::state::ForecastState;
use crate::types::DatasetGroup;

/// Serializable view of the entire Forecast state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ForecastStateView {
    /// Dataset groups keyed by ARN.
    #[serde(default)]
    pub dataset_groups: HashMap<String, DatasetGroupView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetGroupView {
    pub dataset_group_name: String,
    pub dataset_group_arn: String,
    pub domain: String,
    #[serde(default)]
    pub dataset_arns: Vec<String>,
    pub status: String,
    pub creation_time: DateTime<Utc>,
    pub last_modification_time: DateTime<Utc>,
}

// --- From internal types to view types ---

impl From<&ForecastState> for ForecastStateView {
    fn from(state: &ForecastState) -> Self {
        ForecastStateView {
            dataset_groups: state
                .dataset_groups
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        DatasetGroupView {
                            dataset_group_name: v.dataset_group_name.clone(),
                            dataset_group_arn: v.dataset_group_arn.clone(),
                            domain: v.domain.clone(),
                            dataset_arns: v.dataset_arns.clone(),
                            status: v.status.clone(),
                            creation_time: v.creation_time,
                            last_modification_time: v.last_modification_time,
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<ForecastStateView> for ForecastState {
    fn from(view: ForecastStateView) -> Self {
        ForecastState {
            dataset_groups: view
                .dataset_groups
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        DatasetGroup {
                            dataset_group_name: v.dataset_group_name,
                            dataset_group_arn: v.dataset_group_arn,
                            domain: v.domain,
                            dataset_arns: v.dataset_arns,
                            status: v.status,
                            creation_time: v.creation_time,
                            last_modification_time: v.last_modification_time,
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for ForecastService {
    type StateView = ForecastStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ForecastStateView::from(&*guard)
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
            *guard = ForecastState::from(view);
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
            let incoming = ForecastState::from(view);
            for (k, v) in incoming.dataset_groups {
                guard.dataset_groups.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

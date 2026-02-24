//! Serde-compatible view types for RDS Data state snapshots.

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::RdsDataService;
use crate::state::RdsDataState;

/// Serializable view of the entire RDS Data state for one account/region.
/// Note: The result_queue is transient (consumed on use), so it is not persisted.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RdsDataStateView {}

// --- From internal types to view types ---

impl From<&RdsDataState> for RdsDataStateView {
    fn from(_state: &RdsDataState) -> Self {
        RdsDataStateView {}
    }
}

// --- From view types to internal types ---

impl From<RdsDataStateView> for RdsDataState {
    fn from(_view: RdsDataStateView) -> Self {
        RdsDataState::default()
    }
}

// --- StatefulService implementation ---

impl StatefulService for RdsDataService {
    type StateView = RdsDataStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        RdsDataStateView::from(&*guard)
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
            *guard = RdsDataState::from(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        _view: Self::StateView,
    ) -> Result<(), StateViewError> {
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

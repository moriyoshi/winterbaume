//! Serde-compatible view types for DynamoDB Streams state snapshots.
//!
//! Stream existence is derived from DynamoDB table state, so there is nothing
//! durable to snapshot here. Shard iterators are transient and are not
//! persisted across restores.

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::DynamoDbStreamsService;
use crate::state::DynamoDbStreamsState;

/// Serializable view of DynamoDB Streams state for one account/region.
///
/// This view is intentionally empty: streams are derived from DynamoDB table
/// state, and shard iterators are transient.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DynamoDbStreamsStateView {}

impl From<&DynamoDbStreamsState> for DynamoDbStreamsStateView {
    fn from(_state: &DynamoDbStreamsState) -> Self {
        DynamoDbStreamsStateView {}
    }
}

impl From<DynamoDbStreamsStateView> for DynamoDbStreamsState {
    fn from(_view: DynamoDbStreamsStateView) -> Self {
        DynamoDbStreamsState::default()
    }
}

impl StatefulService for DynamoDbStreamsService {
    type StateView = DynamoDbStreamsStateView;

    async fn snapshot(&self, _account_id: &str, _region: &str) -> Self::StateView {
        DynamoDbStreamsStateView {}
    }

    async fn restore(
        &self,
        _account_id: &str,
        _region: &str,
        _view: Self::StateView,
    ) -> Result<(), StateViewError> {
        Ok(())
    }

    async fn merge(
        &self,
        _account_id: &str,
        _region: &str,
        _view: Self::StateView,
    ) -> Result<(), StateViewError> {
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

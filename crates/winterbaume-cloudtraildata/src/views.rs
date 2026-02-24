use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::CloudTrailDataService;
use crate::state::{CloudTrailDataState, EventRecord};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CloudTrailDataStateView {
    #[serde(default)]
    pub events: Vec<EventRecordView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventRecordView {
    pub channel_arn: String,
    pub event_id: String,
    pub external_id: String,
    pub event_data: Value,
    #[serde(default)]
    pub checksum: Option<String>,
}

impl From<&EventRecord> for EventRecordView {
    fn from(e: &EventRecord) -> Self {
        Self {
            channel_arn: e.channel_arn.clone(),
            event_id: e.event_id.clone(),
            external_id: e.external_id.clone(),
            event_data: e.event_data.clone(),
            checksum: e.checksum.clone(),
        }
    }
}

impl From<EventRecordView> for EventRecord {
    fn from(v: EventRecordView) -> Self {
        Self {
            channel_arn: v.channel_arn,
            event_id: v.event_id,
            external_id: v.external_id,
            event_data: v.event_data,
            checksum: v.checksum,
        }
    }
}

impl From<&CloudTrailDataState> for CloudTrailDataStateView {
    fn from(state: &CloudTrailDataState) -> Self {
        Self {
            events: state.events.iter().map(|e| e.into()).collect(),
        }
    }
}

impl From<CloudTrailDataStateView> for CloudTrailDataState {
    fn from(view: CloudTrailDataStateView) -> Self {
        Self {
            events: view.events.into_iter().map(|e| e.into()).collect(),
        }
    }
}

impl StatefulService for CloudTrailDataService {
    type StateView = CloudTrailDataStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        CloudTrailDataStateView::from(&*guard)
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
            *guard = CloudTrailDataState::from(view);
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
        let merged = CloudTrailDataState::from(view);
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            guard.events.extend(merged.events);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ChimeSdkMeetingsService;
use crate::state::{ChimeSdkMeetingsState, MeetingRecord};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChimeSdkMeetingsStateView {
    #[serde(default)]
    pub meetings: HashMap<String, MeetingRecordView>,
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeetingRecordView {
    #[serde(default)]
    pub meeting: Value,
    #[serde(default)]
    pub attendees: HashMap<String, Value>,
    #[serde(default)]
    pub transcription_active: bool,
}

impl From<&MeetingRecord> for MeetingRecordView {
    fn from(r: &MeetingRecord) -> Self {
        Self {
            meeting: r.meeting.clone(),
            attendees: r.attendees.clone(),
            transcription_active: r.transcription_active,
        }
    }
}

impl From<MeetingRecordView> for MeetingRecord {
    fn from(v: MeetingRecordView) -> Self {
        Self {
            meeting: v.meeting,
            attendees: v.attendees,
            transcription_active: v.transcription_active,
        }
    }
}

impl From<&ChimeSdkMeetingsState> for ChimeSdkMeetingsStateView {
    fn from(s: &ChimeSdkMeetingsState) -> Self {
        Self {
            meetings: s
                .meetings
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            tags: s.tags.clone(),
        }
    }
}

impl From<ChimeSdkMeetingsStateView> for ChimeSdkMeetingsState {
    fn from(v: ChimeSdkMeetingsStateView) -> Self {
        Self {
            meetings: v.meetings.into_iter().map(|(k, v)| (k, v.into())).collect(),
            tags: v.tags,
        }
    }
}

impl StatefulService for ChimeSdkMeetingsService {
    type StateView = ChimeSdkMeetingsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ChimeSdkMeetingsStateView::from(&*guard)
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
            *guard = ChimeSdkMeetingsState::from(view);
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
            for (k, v) in view.meetings {
                guard.meetings.insert(k, v.into());
            }
            for (k, v) in view.tags {
                guard.tags.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

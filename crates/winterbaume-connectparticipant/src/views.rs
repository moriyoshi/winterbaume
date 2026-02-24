use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ConnectParticipantService;
use crate::state::{AttachmentRecord, ConnectParticipantState, ConnectionRecord};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectParticipantStateView {
    #[serde(default)]
    pub connections: HashMap<String, ConnectionRecordView>,
    #[serde(default)]
    pub transcripts: HashMap<String, Vec<Value>>,
    #[serde(default)]
    pub attachments: HashMap<String, AttachmentRecordView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectionRecordView {
    pub participant_token: String,
    pub connection_token: String,
    pub disconnected: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttachmentRecordView {
    pub attachment_id: String,
    pub connection_token: String,
    pub name: String,
    pub size_in_bytes: i64,
    pub content_type: String,
    pub status: String,
}

macro_rules! basic_from {
    ($view:ident, $domain:ident { $($field:ident),* $(,)? }) => {
        impl From<&$domain> for $view {
            fn from(s: &$domain) -> Self { Self { $($field: s.$field.clone(),)* } }
        }
        impl From<$view> for $domain {
            fn from(v: $view) -> Self { Self { $($field: v.$field,)* } }
        }
    };
}

basic_from!(
    ConnectionRecordView,
    ConnectionRecord {
        participant_token,
        connection_token,
        disconnected,
    }
);

basic_from!(
    AttachmentRecordView,
    AttachmentRecord {
        attachment_id,
        connection_token,
        name,
        size_in_bytes,
        content_type,
        status,
    }
);

impl From<&ConnectParticipantState> for ConnectParticipantStateView {
    fn from(state: &ConnectParticipantState) -> Self {
        Self {
            connections: state
                .connections
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            transcripts: state.transcripts.clone(),
            attachments: state
                .attachments
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
        }
    }
}

impl From<ConnectParticipantStateView> for ConnectParticipantState {
    fn from(view: ConnectParticipantStateView) -> Self {
        Self {
            connections: view
                .connections
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            transcripts: view.transcripts,
            attachments: view
                .attachments
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

impl StatefulService for ConnectParticipantService {
    type StateView = ConnectParticipantStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ConnectParticipantStateView::from(&*guard)
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
            *guard = ConnectParticipantState::from(view);
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
        let merged = ConnectParticipantState::from(view);
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (k, v) in merged.connections {
                guard.connections.insert(k, v);
            }
            for (k, mut v) in merged.transcripts {
                guard.transcripts.entry(k).or_default().append(&mut v);
            }
            for (k, v) in merged.attachments {
                guard.attachments.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

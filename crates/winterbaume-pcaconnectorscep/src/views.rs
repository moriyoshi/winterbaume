use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::PcaConnectorScepService;
use crate::state::{ChallengeRecord, ConnectorRecord, PcaConnectorScepState};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PcaConnectorScepStateView {
    #[serde(default)]
    pub connectors: HashMap<String, ConnectorRecordView>,
    #[serde(default)]
    pub challenges: HashMap<String, ChallengeRecordView>,
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectorRecordView {
    pub arn: String,
    pub certificate_authority_arn: String,
    pub created_at: f64,
    pub updated_at: f64,
    pub status: String,
    pub connector_type: String,
    pub endpoint: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChallengeRecordView {
    pub arn: String,
    pub connector_arn: String,
    pub password: String,
    pub created_at: f64,
    pub updated_at: f64,
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
    ConnectorRecordView,
    ConnectorRecord {
        arn,
        certificate_authority_arn,
        created_at,
        updated_at,
        status,
        connector_type,
        endpoint,
    }
);

basic_from!(
    ChallengeRecordView,
    ChallengeRecord {
        arn,
        connector_arn,
        password,
        created_at,
        updated_at,
    }
);

impl From<&PcaConnectorScepState> for PcaConnectorScepStateView {
    fn from(state: &PcaConnectorScepState) -> Self {
        Self {
            connectors: state
                .connectors
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            challenges: state
                .challenges
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            tags: state.tags.clone(),
        }
    }
}

impl From<PcaConnectorScepStateView> for PcaConnectorScepState {
    fn from(view: PcaConnectorScepStateView) -> Self {
        Self {
            connectors: view
                .connectors
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            challenges: view
                .challenges
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            tags: view.tags,
        }
    }
}

impl StatefulService for PcaConnectorScepService {
    type StateView = PcaConnectorScepStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        PcaConnectorScepStateView::from(&*guard)
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
            *guard = PcaConnectorScepState::from(view);
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
        let merged = PcaConnectorScepState::from(view);
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (k, v) in merged.connectors {
                guard.connectors.insert(k, v);
            }
            for (k, v) in merged.challenges {
                guard.challenges.insert(k, v);
            }
            for (k, v) in merged.tags {
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

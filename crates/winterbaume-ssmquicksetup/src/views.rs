use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::SsmQuickSetupService;
use crate::state::{ManagerRecord, ServiceSettingsRecord, SsmQuickSetupState};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SsmQuickSetupStateView {
    #[serde(default)]
    pub managers: HashMap<String, ManagerRecordView>,
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
    #[serde(default)]
    pub service_settings: ServiceSettingsRecordView,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ManagerRecordView {
    pub arn: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    pub created_at: String,
    pub last_modified_at: String,
    pub status_summary: String,
    #[serde(default)]
    pub definitions: Vec<Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServiceSettingsRecordView {
    #[serde(default)]
    pub explorer_enabling_role_arn: Option<String>,
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
    ManagerRecordView,
    ManagerRecord {
        arn,
        name,
        description,
        created_at,
        last_modified_at,
        status_summary,
        definitions,
    }
);

basic_from!(
    ServiceSettingsRecordView,
    ServiceSettingsRecord {
        explorer_enabling_role_arn,
    }
);

impl From<&SsmQuickSetupState> for SsmQuickSetupStateView {
    fn from(state: &SsmQuickSetupState) -> Self {
        Self {
            managers: state
                .managers
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            tags: state.tags.clone(),
            service_settings: (&state.service_settings).into(),
        }
    }
}

impl From<SsmQuickSetupStateView> for SsmQuickSetupState {
    fn from(view: SsmQuickSetupStateView) -> Self {
        Self {
            managers: view
                .managers
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            tags: view.tags,
            service_settings: view.service_settings.into(),
        }
    }
}

impl StatefulService for SsmQuickSetupService {
    type StateView = SsmQuickSetupStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        SsmQuickSetupStateView::from(&*guard)
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
            *guard = SsmQuickSetupState::from(view);
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
        let merged = SsmQuickSetupState::from(view);
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (k, v) in merged.managers {
                guard.managers.insert(k, v);
            }
            for (k, v) in merged.tags {
                guard.tags.insert(k, v);
            }
            if merged.service_settings.explorer_enabling_role_arn.is_some() {
                guard.service_settings = merged.service_settings;
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

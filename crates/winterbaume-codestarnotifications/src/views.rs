use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::CodeStarNotificationsService;
use crate::state::{CodeStarNotificationsState, RuleRecord, TargetRecord};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CodeStarNotificationsStateView {
    #[serde(default)]
    pub rules: HashMap<String, RuleRecordView>,
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RuleRecordView {
    pub arn: String,
    pub name: String,
    pub resource: String,
    pub detail_type: String,
    #[serde(default)]
    pub event_type_ids: Vec<String>,
    pub status: String,
    #[serde(default)]
    pub targets: Vec<TargetRecordView>,
    #[serde(default)]
    pub created_timestamp: f64,
    #[serde(default)]
    pub last_modified_timestamp: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TargetRecordView {
    pub target_address: String,
    pub target_type: String,
    pub target_status: String,
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
    TargetRecordView,
    TargetRecord {
        target_address,
        target_type,
        target_status,
    }
);

impl From<&RuleRecord> for RuleRecordView {
    fn from(s: &RuleRecord) -> Self {
        Self {
            arn: s.arn.clone(),
            name: s.name.clone(),
            resource: s.resource.clone(),
            detail_type: s.detail_type.clone(),
            event_type_ids: s.event_type_ids.clone(),
            status: s.status.clone(),
            targets: s.targets.iter().map(|t| t.into()).collect(),
            created_timestamp: s.created_timestamp,
            last_modified_timestamp: s.last_modified_timestamp,
        }
    }
}

impl From<RuleRecordView> for RuleRecord {
    fn from(v: RuleRecordView) -> Self {
        Self {
            arn: v.arn,
            name: v.name,
            resource: v.resource,
            detail_type: v.detail_type,
            event_type_ids: v.event_type_ids,
            status: v.status,
            targets: v.targets.into_iter().map(|t| t.into()).collect(),
            created_timestamp: v.created_timestamp,
            last_modified_timestamp: v.last_modified_timestamp,
        }
    }
}

impl From<&CodeStarNotificationsState> for CodeStarNotificationsStateView {
    fn from(s: &CodeStarNotificationsState) -> Self {
        Self {
            rules: s.rules.iter().map(|(k, v)| (k.clone(), v.into())).collect(),
            tags: s.tags.clone(),
        }
    }
}

impl From<CodeStarNotificationsStateView> for CodeStarNotificationsState {
    fn from(v: CodeStarNotificationsStateView) -> Self {
        Self {
            rules: v.rules.into_iter().map(|(k, r)| (k, r.into())).collect(),
            tags: v.tags,
        }
    }
}

impl StatefulService for CodeStarNotificationsService {
    type StateView = CodeStarNotificationsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        CodeStarNotificationsStateView::from(&*guard)
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
            *guard = CodeStarNotificationsState::from(view);
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
            for (k, v) in view.rules {
                guard.rules.insert(k, v.into());
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

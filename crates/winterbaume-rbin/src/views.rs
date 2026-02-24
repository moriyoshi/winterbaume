use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::RbinService;
use crate::state::{RbinState, RuleRecord};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RbinStateView {
    #[serde(default)]
    pub rules: HashMap<String, RuleRecordView>,
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RuleRecordView {
    pub identifier: String,
    #[serde(default)]
    pub description: Option<String>,
    pub resource_type: String,
    #[serde(default)]
    pub retention_period: Option<Value>,
    #[serde(default)]
    pub resource_tags: Vec<Value>,
    #[serde(default)]
    pub exclude_resource_tags: Vec<Value>,
    pub status: String,
    #[serde(default)]
    pub lock_state: Option<String>,
    #[serde(default)]
    pub lock_configuration: Option<Value>,
    #[serde(default)]
    pub lock_end_time: Option<f64>,
    pub rule_arn: String,
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
    RuleRecordView,
    RuleRecord {
        identifier,
        description,
        resource_type,
        retention_period,
        resource_tags,
        exclude_resource_tags,
        status,
        lock_state,
        lock_configuration,
        lock_end_time,
        rule_arn
    }
);

impl From<&RbinState> for RbinStateView {
    fn from(state: &RbinState) -> Self {
        Self {
            rules: state
                .rules
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            tags: state.tags.clone(),
        }
    }
}

impl From<RbinStateView> for RbinState {
    fn from(view: RbinStateView) -> Self {
        Self {
            rules: view.rules.into_iter().map(|(k, v)| (k, v.into())).collect(),
            tags: view.tags,
        }
    }
}

impl StatefulService for RbinService {
    type StateView = RbinStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        RbinStateView::from(&*guard)
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
            *guard = RbinState::from(view);
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

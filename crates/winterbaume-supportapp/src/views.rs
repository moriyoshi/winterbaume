use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::SupportAppService;
use crate::state::{ChannelConfig, SupportAppState, WorkspaceConfig};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SupportAppStateView {
    /// Channel configurations keyed by "{team_id}/{channel_id}".
    #[serde(default)]
    pub channel_configurations: HashMap<String, ChannelConfigView>,
    #[serde(default)]
    pub workspace_configurations: HashMap<String, WorkspaceConfigView>,
    #[serde(default)]
    pub account_alias: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChannelConfigView {
    pub channel_id: String,
    #[serde(default)]
    pub channel_name: Option<String>,
    pub channel_role_arn: String,
    pub team_id: String,
    pub notify_on_case_severity: String,
    #[serde(default)]
    pub notify_on_add_correspondence_to_case: Option<bool>,
    #[serde(default)]
    pub notify_on_create_or_reopen_case: Option<bool>,
    #[serde(default)]
    pub notify_on_resolve_case: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkspaceConfigView {
    pub team_id: String,
    #[serde(default)]
    pub team_name: Option<String>,
    #[serde(default)]
    pub allow_organization_member_account: Option<bool>,
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
    ChannelConfigView,
    ChannelConfig {
        channel_id,
        channel_name,
        channel_role_arn,
        team_id,
        notify_on_case_severity,
        notify_on_add_correspondence_to_case,
        notify_on_create_or_reopen_case,
        notify_on_resolve_case,
    }
);

basic_from!(
    WorkspaceConfigView,
    WorkspaceConfig {
        team_id,
        team_name,
        allow_organization_member_account,
    }
);

impl From<&SupportAppState> for SupportAppStateView {
    fn from(state: &SupportAppState) -> Self {
        Self {
            channel_configurations: state
                .channel_configurations
                .iter()
                .map(|((t, c), v)| (format!("{t}/{c}"), v.into()))
                .collect(),
            workspace_configurations: state
                .workspace_configurations
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            account_alias: state.account_alias.clone(),
        }
    }
}

impl From<SupportAppStateView> for SupportAppState {
    fn from(view: SupportAppStateView) -> Self {
        let channels = view
            .channel_configurations
            .into_iter()
            .filter_map(|(k, v)| {
                let cfg: ChannelConfig = v.into();
                Some(((cfg.team_id.clone(), cfg.channel_id.clone()), cfg))
                    .filter(|((_, _), _)| !k.is_empty())
            })
            .collect();
        let workspaces = view
            .workspace_configurations
            .into_iter()
            .map(|(k, v)| (k, v.into()))
            .collect();
        Self {
            channel_configurations: channels,
            workspace_configurations: workspaces,
            account_alias: view.account_alias,
        }
    }
}

impl StatefulService for SupportAppService {
    type StateView = SupportAppStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        SupportAppStateView::from(&*guard)
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
            *guard = SupportAppState::from(view);
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
        let merged = SupportAppState::from(view);
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (k, v) in merged.channel_configurations {
                guard.channel_configurations.insert(k, v);
            }
            for (k, v) in merged.workspace_configurations {
                guard.workspace_configurations.insert(k, v);
            }
            if let Some(a) = merged.account_alias {
                guard.account_alias = Some(a);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

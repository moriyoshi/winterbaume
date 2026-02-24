//! Serde-compatible view types for Connect state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ConnectService;
use crate::state::ConnectState;

/// Serializable view of the entire Connect state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectStateView {
    #[serde(default)]
    pub instances: HashMap<String, ConnectInstanceView>,
    /// Analytics data associations as a list (tuple key flattened).
    #[serde(default)]
    pub analytics_data_associations: Vec<AnalyticsDataAssociationView>,
    #[serde(default)]
    pub resource_tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectInstanceView {
    pub id: String,
    pub arn: String,
    pub identity_management_type: String,
    pub instance_alias: Option<String>,
    pub instance_status: String,
    pub created_time: String,
    pub inbound_calls_enabled: bool,
    pub outbound_calls_enabled: bool,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsDataAssociationView {
    pub instance_id: String,
    pub data_set_id: String,
    pub target_account_id: Option<String>,
    pub resource_share_id: String,
    pub resource_share_arn: String,
}

fn parse_datetime(s: &str) -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .unwrap_or_else(|_| chrono::Utc::now())
}

// --- From internal types to view types ---

impl From<&ConnectState> for ConnectStateView {
    fn from(state: &ConnectState) -> Self {
        ConnectStateView {
            instances: state
                .instances
                .iter()
                .map(|(k, inst)| {
                    (
                        k.clone(),
                        ConnectInstanceView {
                            id: inst.id.clone(),
                            arn: inst.arn.clone(),
                            identity_management_type: inst.identity_management_type.clone(),
                            instance_alias: inst.instance_alias.clone(),
                            instance_status: inst.instance_status.clone(),
                            created_time: inst.created_time.to_rfc3339(),
                            inbound_calls_enabled: inst.inbound_calls_enabled,
                            outbound_calls_enabled: inst.outbound_calls_enabled,
                            tags: inst.tags.clone(),
                        },
                    )
                })
                .collect(),
            analytics_data_associations: state
                .analytics_data_associations
                .values()
                .map(|a| AnalyticsDataAssociationView {
                    instance_id: a.instance_id.clone(),
                    data_set_id: a.data_set_id.clone(),
                    target_account_id: a.target_account_id.clone(),
                    resource_share_id: a.resource_share_id.clone(),
                    resource_share_arn: a.resource_share_arn.clone(),
                })
                .collect(),
            resource_tags: state.resource_tags.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<ConnectStateView> for ConnectState {
    fn from(view: ConnectStateView) -> Self {
        let mut state = ConnectState::default();
        for (k, inst) in view.instances {
            state.instances.insert(
                k,
                crate::types::ConnectInstance {
                    id: inst.id,
                    arn: inst.arn,
                    identity_management_type: inst.identity_management_type,
                    instance_alias: inst.instance_alias,
                    instance_status: inst.instance_status,
                    created_time: parse_datetime(&inst.created_time),
                    inbound_calls_enabled: inst.inbound_calls_enabled,
                    outbound_calls_enabled: inst.outbound_calls_enabled,
                    tags: inst.tags,
                },
            );
        }
        for a in view.analytics_data_associations {
            let key = (a.instance_id.clone(), a.data_set_id.clone());
            state.analytics_data_associations.insert(
                key,
                crate::types::AnalyticsDataAssociation {
                    instance_id: a.instance_id,
                    data_set_id: a.data_set_id,
                    target_account_id: a.target_account_id,
                    resource_share_id: a.resource_share_id,
                    resource_share_arn: a.resource_share_arn,
                },
            );
        }
        state.resource_tags = view.resource_tags;
        state
    }
}

// --- StatefulService implementation ---

impl StatefulService for ConnectService {
    type StateView = ConnectStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ConnectStateView::from(&*guard)
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
            *guard = ConnectState::from(view);
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
            let merged = ConnectState::from(view);
            for (k, v) in merged.instances {
                guard.instances.insert(k, v);
            }
            for (k, v) in merged.analytics_data_associations {
                guard.analytics_data_associations.insert(k, v);
            }
            for (k, v) in merged.resource_tags {
                guard.resource_tags.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

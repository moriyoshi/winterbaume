//! Serde-compatible view types for Direct Connect state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::DirectConnectService;
use crate::state::DirectConnectState;
use crate::types::{Connection, ConnectionState};

/// Serializable view of the entire Direct Connect state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DirectConnectStateView {
    /// Connections keyed by connection ID.
    #[serde(default)]
    pub connections: HashMap<String, ConnectionView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionView {
    pub connection_id: String,
    pub connection_name: String,
    /// Connection state as string (e.g., "requested", "available", etc.)
    pub connection_state: String,
    pub region: String,
    pub location: String,
    pub bandwidth: String,
    pub owner_account: String,
    pub vlan: i32,
    pub partner_name: Option<String>,
    pub loa_issue_time: Option<DateTime<Utc>>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

// --- From internal types to view types ---

impl From<&DirectConnectState> for DirectConnectStateView {
    fn from(state: &DirectConnectState) -> Self {
        DirectConnectStateView {
            connections: state
                .connections
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ConnectionView {
                            connection_id: v.connection_id.clone(),
                            connection_name: v.connection_name.clone(),
                            connection_state: v.connection_state.as_str().to_string(),
                            region: v.region.clone(),
                            location: v.location.clone(),
                            bandwidth: v.bandwidth.clone(),
                            owner_account: v.owner_account.clone(),
                            vlan: v.vlan,
                            partner_name: v.partner_name.clone(),
                            loa_issue_time: v.loa_issue_time,
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

fn parse_connection_state(s: &str) -> ConnectionState {
    match s {
        "ordering" => ConnectionState::Ordering,
        "requested" => ConnectionState::Requested,
        "pending" => ConnectionState::Pending,
        "available" => ConnectionState::Available,
        "down" => ConnectionState::Down,
        "deleting" => ConnectionState::Deleting,
        "deleted" => ConnectionState::Deleted,
        "rejected" => ConnectionState::Rejected,
        _ => ConnectionState::Unknown,
    }
}

// --- From view types to internal types ---

impl From<DirectConnectStateView> for DirectConnectState {
    fn from(view: DirectConnectStateView) -> Self {
        DirectConnectState {
            connections: view
                .connections
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Connection {
                            connection_id: v.connection_id,
                            connection_name: v.connection_name,
                            connection_state: parse_connection_state(&v.connection_state),
                            region: v.region,
                            location: v.location,
                            bandwidth: v.bandwidth,
                            owner_account: v.owner_account,
                            vlan: v.vlan,
                            partner_name: v.partner_name,
                            loa_issue_time: v.loa_issue_time,
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for DirectConnectService {
    type StateView = DirectConnectStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        DirectConnectStateView::from(&*guard)
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
            *guard = DirectConnectState::from(view);
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
            let incoming = DirectConnectState::from(view);
            for (k, v) in incoming.connections {
                guard.connections.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

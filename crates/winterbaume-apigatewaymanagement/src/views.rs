use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ApiGatewayManagementService;
use crate::state::ApiGatewayManagementApiState;
use crate::types::Connection;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiGatewayManagementApiStateView {
    #[serde(default)]
    pub connections: HashMap<String, ConnectionView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionView {
    pub connected_at: DateTime<Utc>,
    pub last_active_at: DateTime<Utc>,
    pub source_ip: String,
    pub user_agent: String,
    pub data: Vec<u8>,
}

impl From<&ApiGatewayManagementApiState> for ApiGatewayManagementApiStateView {
    fn from(state: &ApiGatewayManagementApiState) -> Self {
        Self {
            connections: state
                .connections
                .iter()
                .map(|(k, v)| (k.clone(), ConnectionView::from(v)))
                .collect(),
        }
    }
}

impl From<&Connection> for ConnectionView {
    fn from(c: &Connection) -> Self {
        Self {
            connected_at: c.connected_at,
            last_active_at: c.last_active_at,
            source_ip: c.source_ip.clone(),
            user_agent: c.user_agent.clone(),
            data: c.data.clone(),
        }
    }
}

impl From<ApiGatewayManagementApiStateView> for ApiGatewayManagementApiState {
    fn from(view: ApiGatewayManagementApiStateView) -> Self {
        Self {
            connections: view
                .connections
                .into_iter()
                .map(|(k, v)| (k, Connection::from(v)))
                .collect(),
        }
    }
}

impl From<ConnectionView> for Connection {
    fn from(v: ConnectionView) -> Self {
        Self {
            connected_at: v.connected_at,
            last_active_at: v.last_active_at,
            source_ip: v.source_ip,
            user_agent: v.user_agent,
            data: v.data,
        }
    }
}

impl StatefulService for ApiGatewayManagementService {
    type StateView = ApiGatewayManagementApiStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ApiGatewayManagementApiStateView::from(&*guard)
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
            *guard = ApiGatewayManagementApiState::from(view);
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
            for (id, conn_view) in view.connections {
                guard.connections.insert(id, Connection::from(conn_view));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

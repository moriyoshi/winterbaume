//! Serde-compatible view types for SSO state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::SsoService;
use crate::state::SsoState;
use crate::types::{SsoAccount, SsoAccountRole, SsoSession};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SsoStateView {
    #[serde(default)]
    pub accounts: HashMap<String, SsoAccountView>,
    /// Roles stored as a flat list because tuple keys cannot be serialized as JSON object keys.
    #[serde(default)]
    pub roles: Vec<SsoAccountRoleView>,
    #[serde(default)]
    pub sessions: HashMap<String, SsoSessionView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoAccountView {
    pub account_id: String,
    pub account_name: String,
    pub email_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoAccountRoleView {
    pub account_id: String,
    pub role_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoSessionView {
    pub access_token: String,
    pub logged_out: bool,
    pub created_at: DateTime<Utc>,
}

impl From<&SsoState> for SsoStateView {
    fn from(state: &SsoState) -> Self {
        SsoStateView {
            accounts: state
                .accounts
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        SsoAccountView {
                            account_id: v.account_id.clone(),
                            account_name: v.account_name.clone(),
                            email_address: v.email_address.clone(),
                        },
                    )
                })
                .collect(),
            roles: state
                .roles
                .values()
                .map(|v| SsoAccountRoleView {
                    account_id: v.account_id.clone(),
                    role_name: v.role_name.clone(),
                })
                .collect(),
            sessions: state
                .sessions
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        SsoSessionView {
                            access_token: v.access_token.clone(),
                            logged_out: v.logged_out,
                            created_at: v.created_at,
                        },
                    )
                })
                .collect(),
        }
    }
}

impl From<SsoStateView> for SsoState {
    fn from(view: SsoStateView) -> Self {
        SsoState {
            accounts: view
                .accounts
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        SsoAccount {
                            account_id: v.account_id,
                            account_name: v.account_name,
                            email_address: v.email_address,
                        },
                    )
                })
                .collect(),
            roles: view
                .roles
                .into_iter()
                .map(|v| {
                    (
                        (v.account_id.clone(), v.role_name.clone()),
                        SsoAccountRole {
                            account_id: v.account_id,
                            role_name: v.role_name,
                        },
                    )
                })
                .collect(),
            sessions: view
                .sessions
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        SsoSession {
                            access_token: v.access_token,
                            logged_out: v.logged_out,
                            created_at: v.created_at,
                        },
                    )
                })
                .collect(),
        }
    }
}

impl StatefulService for SsoService {
    type StateView = SsoStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        SsoStateView::from(&*guard)
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
            *guard = SsoState::from(view);
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
            for (k, v) in view.accounts {
                guard.accounts.insert(
                    k,
                    SsoAccount {
                        account_id: v.account_id,
                        account_name: v.account_name,
                        email_address: v.email_address,
                    },
                );
            }
            for v in view.roles {
                guard.roles.insert(
                    (v.account_id.clone(), v.role_name.clone()),
                    SsoAccountRole {
                        account_id: v.account_id,
                        role_name: v.role_name,
                    },
                );
            }
            for (k, v) in view.sessions {
                guard.sessions.insert(
                    k,
                    SsoSession {
                        access_token: v.access_token,
                        logged_out: v.logged_out,
                        created_at: v.created_at,
                    },
                );
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

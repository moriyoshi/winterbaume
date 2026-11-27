//! Serde-compatible view types for STS state snapshots.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::StsService;
use crate::state::StsState;
use crate::types::{AssumedRole, CallerIdentity, CallerSession};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StsStateView {
    #[serde(default)]
    pub assumed_roles: Vec<AssumedRoleView>,
    /// Credentials issued for a caller's own identity ( `AssumeRole` without
    /// `RoleArn`, under the `sts-assume-role-without-role-arn` relaxation ).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub caller_sessions: Vec<CallerSessionView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallerSessionView {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: String,
    pub expiration: DateTime<Utc>,
    pub user_id: String,
    pub account: String,
    pub arn: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_policy: Option<String>,
}

impl From<&CallerSession> for CallerSessionView {
    fn from(s: &CallerSession) -> Self {
        CallerSessionView {
            access_key_id: s.access_key_id.clone(),
            secret_access_key: s.secret_access_key.clone(),
            session_token: s.session_token.clone(),
            expiration: s.expiration,
            user_id: s.identity.user_id.clone(),
            account: s.identity.account.clone(),
            arn: s.identity.arn.clone(),
            session_policy: s.session_policy.clone(),
        }
    }
}

impl From<CallerSessionView> for CallerSession {
    fn from(v: CallerSessionView) -> Self {
        CallerSession {
            access_key_id: v.access_key_id,
            secret_access_key: v.secret_access_key,
            session_token: v.session_token,
            expiration: v.expiration,
            identity: CallerIdentity {
                user_id: v.user_id,
                account: v.account,
                arn: v.arn,
            },
            session_policy: v.session_policy,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssumedRoleView {
    pub account_id: String,
    pub role_arn: String,
    pub role_session_name: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: String,
    pub expiration: DateTime<Utc>,
    pub assumed_role_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_policy: Option<String>,
}

impl From<&StsState> for StsStateView {
    fn from(state: &StsState) -> Self {
        StsStateView {
            assumed_roles: state
                .assumed_roles
                .iter()
                .map(|r| AssumedRoleView {
                    account_id: r.account_id.clone(),
                    role_arn: r.role_arn.clone(),
                    role_session_name: r.role_session_name.clone(),
                    access_key_id: r.access_key_id.clone(),
                    secret_access_key: r.secret_access_key.clone(),
                    session_token: r.session_token.clone(),
                    expiration: r.expiration,
                    assumed_role_id: r.assumed_role_id.clone(),
                    session_policy: r.session_policy.clone(),
                })
                .collect(),
            caller_sessions: state
                .caller_sessions
                .iter()
                .map(CallerSessionView::from)
                .collect(),
        }
    }
}

impl From<StsStateView> for StsState {
    fn from(view: StsStateView) -> Self {
        StsState {
            assumed_roles: view
                .assumed_roles
                .into_iter()
                .map(|r| AssumedRole {
                    account_id: r.account_id,
                    role_arn: r.role_arn,
                    role_session_name: r.role_session_name,
                    access_key_id: r.access_key_id,
                    secret_access_key: r.secret_access_key,
                    session_token: r.session_token,
                    expiration: r.expiration,
                    assumed_role_id: r.assumed_role_id,
                    session_policy: r.session_policy,
                })
                .collect(),
            caller_sessions: view
                .caller_sessions
                .into_iter()
                .map(CallerSession::from)
                .collect(),
        }
    }
}

impl StatefulService for StsService {
    type StateView = StsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        StsStateView::from(&*guard)
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
            *guard = StsState::from(view);
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
            for r in view.assumed_roles {
                guard.assumed_roles.push(AssumedRole {
                    account_id: r.account_id,
                    role_arn: r.role_arn,
                    role_session_name: r.role_session_name,
                    access_key_id: r.access_key_id,
                    secret_access_key: r.secret_access_key,
                    session_token: r.session_token,
                    expiration: r.expiration,
                    assumed_role_id: r.assumed_role_id,
                    session_policy: r.session_policy,
                });
            }
            guard
                .caller_sessions
                .extend(view.caller_sessions.into_iter().map(CallerSession::from));
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct SsoState {
    /// Accounts keyed by account_id.
    pub accounts: HashMap<String, SsoAccount>,
    /// Roles keyed by (account_id, role_name).
    pub roles: HashMap<(String, String), SsoAccountRole>,
    /// Sessions keyed by access_token.
    pub sessions: HashMap<String, SsoSession>,
}

#[derive(Debug, Error)]
pub enum SsoError {
    #[error("Invalid access token")]
    InvalidAccessToken,
    #[error("Session has been invalidated")]
    SessionInvalidated,
    #[error("Account {account_id} not found")]
    AccountNotFound { account_id: String },
    #[error("Role {role_name} not found for account {account_id}")]
    RoleNotFound {
        role_name: String,
        account_id: String,
    },
}

impl SsoState {
    /// Register an account visible through SSO.
    pub fn add_account(&mut self, account_id: &str, account_name: &str, email_address: &str) {
        let account = SsoAccount {
            account_id: account_id.to_string(),
            account_name: account_name.to_string(),
            email_address: email_address.to_string(),
        };
        self.accounts.insert(account_id.to_string(), account);
    }

    /// Register a role for an account.
    pub fn add_role(&mut self, account_id: &str, role_name: &str) {
        let role = SsoAccountRole {
            account_id: account_id.to_string(),
            role_name: role_name.to_string(),
        };
        self.roles
            .insert((account_id.to_string(), role_name.to_string()), role);
    }

    /// Register an SSO access token session.
    pub fn add_session(&mut self, access_token: &str) {
        let session = SsoSession {
            access_token: access_token.to_string(),
            logged_out: false,
            created_at: Utc::now(),
        };
        self.sessions.insert(access_token.to_string(), session);
    }

    /// Validate an access token, returning an error if invalid or logged out.
    pub fn validate_token(&self, token: &str) -> Result<(), SsoError> {
        match self.sessions.get(token) {
            Some(session) if session.logged_out => Err(SsoError::SessionInvalidated),
            Some(_) => Ok(()),
            None => Err(SsoError::InvalidAccessToken),
        }
    }

    /// List all accounts.
    pub fn list_accounts(&self) -> Vec<&SsoAccount> {
        self.accounts.values().collect()
    }

    /// List roles for a specific account.
    pub fn list_account_roles(&self, account_id: &str) -> Result<Vec<&SsoAccountRole>, SsoError> {
        if !self.accounts.contains_key(account_id) {
            return Err(SsoError::AccountNotFound {
                account_id: account_id.to_string(),
            });
        }
        let roles: Vec<&SsoAccountRole> = self
            .roles
            .values()
            .filter(|r| r.account_id == account_id)
            .collect();
        Ok(roles)
    }

    /// Get role credentials for a specific account and role.
    pub fn get_role_credentials(
        &self,
        account_id: &str,
        role_name: &str,
    ) -> Result<SsoRoleCredentials, SsoError> {
        let key = (account_id.to_string(), role_name.to_string());
        if !self.roles.contains_key(&key) {
            return Err(SsoError::RoleNotFound {
                role_name: role_name.to_string(),
                account_id: account_id.to_string(),
            });
        }

        let expiration = Utc::now().timestamp_millis() + 3600 * 1000;
        Ok(SsoRoleCredentials {
            access_key_id: format!("ASIA{}", &uuid::Uuid::new_v4().to_string()[..16]),
            secret_access_key: uuid::Uuid::new_v4().to_string(),
            session_token: format!("FwoGZX...mock-session-token-{}", uuid::Uuid::new_v4()),
            expiration,
        })
    }

    /// Logout (invalidate) a session.
    pub fn logout(&mut self, token: &str) -> Result<(), SsoError> {
        match self.sessions.get_mut(token) {
            Some(session) => {
                session.logged_out = true;
                Ok(())
            }
            None => Err(SsoError::InvalidAccessToken),
        }
    }
}

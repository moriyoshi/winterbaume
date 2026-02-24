use std::collections::HashMap;

use thiserror::Error;
use uuid::Uuid;

use crate::types::*;

#[derive(Debug, Default)]
pub struct AppConfigDataState {
    /// Sessions keyed by their initial / current configuration token.
    pub sessions: HashMap<String, ConfigurationSession>,
}

#[derive(Debug, Error)]
pub enum AppConfigDataError {
    #[error("Configuration token {token} is invalid or expired")]
    BadConfigurationToken { token: String },
}

impl AppConfigDataState {
    pub fn start_session(
        &mut self,
        application_id: &str,
        environment_id: &str,
        configuration_profile_id: &str,
        required_minimum_poll_interval_in_seconds: Option<i32>,
    ) -> &ConfigurationSession {
        let token = Uuid::new_v4().simple().to_string();
        let session = ConfigurationSession {
            token: token.clone(),
            application_id: application_id.to_string(),
            environment_id: environment_id.to_string(),
            configuration_profile_id: configuration_profile_id.to_string(),
            required_minimum_poll_interval_in_seconds,
        };
        self.sessions.insert(token.clone(), session);
        self.sessions.get(&token).unwrap()
    }

    pub fn get_session(&self, token: &str) -> Result<&ConfigurationSession, AppConfigDataError> {
        self.sessions
            .get(token)
            .ok_or_else(|| AppConfigDataError::BadConfigurationToken {
                token: token.to_string(),
            })
    }

    /// Issue a new poll token for the given session, replacing the previous one.
    pub fn rotate_token(&mut self, old_token: &str) -> Result<String, AppConfigDataError> {
        let session = self.sessions.remove(old_token).ok_or_else(|| {
            AppConfigDataError::BadConfigurationToken {
                token: old_token.to_string(),
            }
        })?;
        let new_token = Uuid::new_v4().simple().to_string();
        let new_session = ConfigurationSession {
            token: new_token.clone(),
            ..session
        };
        self.sessions.insert(new_token.clone(), new_session);
        Ok(new_token)
    }
}

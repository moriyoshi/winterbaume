use std::collections::HashMap;

use thiserror::Error;

#[derive(Debug, Default)]
pub struct SupportAppState {
    /// Slack channel configurations keyed by (team_id, channel_id).
    pub channel_configurations: HashMap<(String, String), ChannelConfig>,
    /// Slack workspace configurations keyed by team_id.
    pub workspace_configurations: HashMap<String, WorkspaceConfig>,
    /// Account alias (single value per account-region scope).
    pub account_alias: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ChannelConfig {
    pub channel_id: String,
    pub channel_name: Option<String>,
    pub channel_role_arn: String,
    pub team_id: String,
    pub notify_on_case_severity: String,
    pub notify_on_add_correspondence_to_case: Option<bool>,
    pub notify_on_create_or_reopen_case: Option<bool>,
    pub notify_on_resolve_case: Option<bool>,
}

#[derive(Debug, Clone, Default)]
pub struct WorkspaceConfig {
    pub team_id: String,
    pub team_name: Option<String>,
    pub allow_organization_member_account: Option<bool>,
}

#[derive(Debug, Error)]
pub enum SupportAppError {
    #[error("Slack channel configuration for team {team_id} channel {channel_id} not found.")]
    ChannelNotFound { team_id: String, channel_id: String },

    #[error("Slack workspace configuration for team {team_id} not found.")]
    WorkspaceNotFound { team_id: String },

    #[error("Slack channel configuration for team {team_id} channel {channel_id} already exists.")]
    ChannelAlreadyExists { team_id: String, channel_id: String },

    #[error("{message}")]
    Validation { message: String },
}

impl SupportAppState {
    pub fn create_channel(
        &mut self,
        cfg: ChannelConfig,
    ) -> Result<&ChannelConfig, SupportAppError> {
        let key = (cfg.team_id.clone(), cfg.channel_id.clone());
        if self.channel_configurations.contains_key(&key) {
            return Err(SupportAppError::ChannelAlreadyExists {
                team_id: cfg.team_id.clone(),
                channel_id: cfg.channel_id.clone(),
            });
        }
        // Auto-create the workspace entry if missing.
        self.workspace_configurations
            .entry(cfg.team_id.clone())
            .or_insert_with(|| WorkspaceConfig {
                team_id: cfg.team_id.clone(),
                team_name: None,
                allow_organization_member_account: None,
            });
        self.channel_configurations.insert(key.clone(), cfg);
        Ok(self.channel_configurations.get(&key).unwrap())
    }

    pub fn delete_channel(
        &mut self,
        team_id: &str,
        channel_id: &str,
    ) -> Result<(), SupportAppError> {
        self.channel_configurations
            .remove(&(team_id.to_string(), channel_id.to_string()))
            .ok_or(SupportAppError::ChannelNotFound {
                team_id: team_id.to_string(),
                channel_id: channel_id.to_string(),
            })?;
        Ok(())
    }

    pub fn update_channel(
        &mut self,
        team_id: &str,
        channel_id: &str,
        update: impl FnOnce(&mut ChannelConfig),
    ) -> Result<&ChannelConfig, SupportAppError> {
        let entry = self
            .channel_configurations
            .get_mut(&(team_id.to_string(), channel_id.to_string()))
            .ok_or(SupportAppError::ChannelNotFound {
                team_id: team_id.to_string(),
                channel_id: channel_id.to_string(),
            })?;
        update(entry);
        Ok(entry)
    }

    pub fn list_channels(&self) -> Vec<&ChannelConfig> {
        let mut v: Vec<&ChannelConfig> = self.channel_configurations.values().collect();
        v.sort_by(|a, b| {
            (a.team_id.as_str(), a.channel_id.as_str())
                .cmp(&(b.team_id.as_str(), b.channel_id.as_str()))
        });
        v
    }

    pub fn delete_workspace(&mut self, team_id: &str) -> Result<(), SupportAppError> {
        self.workspace_configurations.remove(team_id).ok_or(
            SupportAppError::WorkspaceNotFound {
                team_id: team_id.to_string(),
            },
        )?;
        // Cascade: drop any channel under this workspace.
        self.channel_configurations.retain(|(t, _), _| t != team_id);
        Ok(())
    }

    pub fn list_workspaces(&self) -> Vec<&WorkspaceConfig> {
        let mut v: Vec<&WorkspaceConfig> = self.workspace_configurations.values().collect();
        v.sort_by(|a, b| a.team_id.cmp(&b.team_id));
        v
    }

    pub fn register_workspace_for_organization(&mut self, team_id: &str) -> &WorkspaceConfig {
        let entry = self
            .workspace_configurations
            .entry(team_id.to_string())
            .or_insert_with(|| WorkspaceConfig {
                team_id: team_id.to_string(),
                team_name: None,
                allow_organization_member_account: Some(true),
            });
        entry.allow_organization_member_account = Some(true);
        entry
    }

    pub fn put_account_alias(&mut self, alias: String) {
        self.account_alias = Some(alias);
    }

    pub fn delete_account_alias(&mut self) {
        self.account_alias = None;
    }
}

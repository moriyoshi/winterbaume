use std::collections::HashMap;

use uuid::Uuid;

use crate::types::{ChimeConfig, SlackConfig, TeamsConfig};

#[derive(Debug, Default)]
pub struct ChatbotState {
    pub slack_configs: HashMap<String, SlackConfig>,
    pub chime_configs: HashMap<String, ChimeConfig>,
    pub teams_configs: HashMap<String, TeamsConfig>,
}

#[derive(Debug, thiserror::Error)]
pub enum ChatbotError {
    #[error("Configuration not found: {0}")]
    ConfigurationNotFound(String),
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),
}

impl ChatbotState {
    #[allow(clippy::too_many_arguments)]
    pub fn create_slack_channel_configuration(
        &mut self,
        account_id: &str,
        region: &str,
        configuration_name: &str,
        slack_team_id: &str,
        slack_channel_id: &str,
        slack_channel_name: Option<String>,
        iam_role_arn: &str,
        sns_topic_arns: Vec<String>,
        logging_level: Option<String>,
        guardrail_policy_arns: Vec<String>,
        user_authorization_required: Option<bool>,
        tags: HashMap<String, String>,
    ) -> Result<SlackConfig, ChatbotError> {
        let arn = format!(
            "arn:aws:chatbot:{}:{}:chat-configuration/slack-channel/{}",
            region, account_id, configuration_name
        );
        let config = SlackConfig {
            arn: arn.clone(),
            configuration_name: configuration_name.to_string(),
            slack_team_id: slack_team_id.to_string(),
            slack_channel_id: slack_channel_id.to_string(),
            slack_channel_name,
            iam_role_arn: iam_role_arn.to_string(),
            sns_topic_arns,
            logging_level,
            guardrail_policy_arns,
            user_authorization_required,
            tags,
        };
        self.slack_configs.insert(arn, config.clone());
        Ok(config)
    }

    pub fn describe_slack_channel_configurations(
        &self,
        filter_arn: Option<&str>,
    ) -> Vec<&SlackConfig> {
        if let Some(arn) = filter_arn {
            self.slack_configs.get(arn).into_iter().collect()
        } else {
            self.slack_configs.values().collect()
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update_slack_channel_configuration(
        &mut self,
        chat_configuration_arn: &str,
        slack_channel_id: &str,
        slack_channel_name: Option<String>,
        iam_role_arn: Option<String>,
        sns_topic_arns: Option<Vec<String>>,
        logging_level: Option<String>,
        guardrail_policy_arns: Option<Vec<String>>,
        user_authorization_required: Option<bool>,
    ) -> Result<SlackConfig, ChatbotError> {
        let config = self
            .slack_configs
            .get_mut(chat_configuration_arn)
            .ok_or_else(|| {
                ChatbotError::ConfigurationNotFound(chat_configuration_arn.to_string())
            })?;
        config.slack_channel_id = slack_channel_id.to_string();
        if let Some(name) = slack_channel_name {
            config.slack_channel_name = Some(name);
        }
        if let Some(iam) = iam_role_arn {
            config.iam_role_arn = iam;
        }
        if let Some(arns) = sns_topic_arns {
            config.sns_topic_arns = arns;
        }
        if let Some(level) = logging_level {
            config.logging_level = Some(level);
        }
        if let Some(arns) = guardrail_policy_arns {
            config.guardrail_policy_arns = arns;
        }
        if let Some(auth) = user_authorization_required {
            config.user_authorization_required = Some(auth);
        }
        Ok(config.clone())
    }

    pub fn delete_slack_channel_configuration(
        &mut self,
        chat_configuration_arn: &str,
    ) -> Result<(), ChatbotError> {
        if self.slack_configs.remove(chat_configuration_arn).is_none() {
            return Err(ChatbotError::ConfigurationNotFound(
                chat_configuration_arn.to_string(),
            ));
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_chime_webhook_configuration(
        &mut self,
        account_id: &str,
        region: &str,
        configuration_name: &str,
        webhook_url: &str,
        webhook_description: &str,
        iam_role_arn: &str,
        sns_topic_arns: Vec<String>,
        logging_level: Option<String>,
        tags: HashMap<String, String>,
    ) -> Result<ChimeConfig, ChatbotError> {
        let id = Uuid::new_v4().to_string();
        let arn = format!(
            "arn:aws:chatbot:{}:{}:chat-configuration/chime-webhook/{}",
            region, account_id, id
        );
        let config = ChimeConfig {
            arn: arn.clone(),
            configuration_name: configuration_name.to_string(),
            webhook_url: webhook_url.to_string(),
            webhook_description: webhook_description.to_string(),
            iam_role_arn: iam_role_arn.to_string(),
            sns_topic_arns,
            logging_level,
            tags,
        };
        self.chime_configs.insert(arn, config.clone());
        Ok(config)
    }

    pub fn describe_chime_webhook_configurations(
        &self,
        filter_arn: Option<&str>,
    ) -> Vec<&ChimeConfig> {
        if let Some(arn) = filter_arn {
            self.chime_configs.get(arn).into_iter().collect()
        } else {
            self.chime_configs.values().collect()
        }
    }

    pub fn delete_chime_webhook_configuration(
        &mut self,
        chat_configuration_arn: &str,
    ) -> Result<(), ChatbotError> {
        if self.chime_configs.remove(chat_configuration_arn).is_none() {
            return Err(ChatbotError::ConfigurationNotFound(
                chat_configuration_arn.to_string(),
            ));
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_microsoft_teams_channel_configuration(
        &mut self,
        account_id: &str,
        region: &str,
        configuration_name: &str,
        team_id: &str,
        team_name: Option<String>,
        tenant_id: &str,
        channel_id: &str,
        channel_name: Option<String>,
        iam_role_arn: &str,
        sns_topic_arns: Vec<String>,
        logging_level: Option<String>,
        guardrail_policy_arns: Vec<String>,
        user_authorization_required: Option<bool>,
        tags: HashMap<String, String>,
    ) -> Result<TeamsConfig, ChatbotError> {
        let arn = format!(
            "arn:aws:chatbot:{}:{}:chat-configuration/microsoft-teams-channel/{}",
            region, account_id, configuration_name
        );
        let config = TeamsConfig {
            arn: arn.clone(),
            configuration_name: configuration_name.to_string(),
            team_id: team_id.to_string(),
            team_name,
            tenant_id: tenant_id.to_string(),
            channel_id: channel_id.to_string(),
            channel_name,
            iam_role_arn: iam_role_arn.to_string(),
            sns_topic_arns,
            logging_level,
            guardrail_policy_arns,
            user_authorization_required,
            tags,
        };
        self.teams_configs.insert(arn, config.clone());
        Ok(config)
    }

    pub fn list_microsoft_teams_channel_configurations(
        &self,
        filter_arn: Option<&str>,
    ) -> Vec<&TeamsConfig> {
        if let Some(arn) = filter_arn {
            self.teams_configs.get(arn).into_iter().collect()
        } else {
            self.teams_configs.values().collect()
        }
    }

    pub fn delete_microsoft_teams_channel_configuration(
        &mut self,
        chat_configuration_arn: &str,
    ) -> Result<(), ChatbotError> {
        if self.teams_configs.remove(chat_configuration_arn).is_none() {
            return Err(ChatbotError::ConfigurationNotFound(
                chat_configuration_arn.to_string(),
            ));
        }
        Ok(())
    }

    // FIX(terraform-e2e): GetMicrosoftTeamsChannelConfiguration needed by terraform
    // provider to read back Teams channel config after create.
    pub fn get_microsoft_teams_channel_configuration(
        &self,
        chat_configuration_arn: &str,
    ) -> Result<&TeamsConfig, ChatbotError> {
        self.teams_configs
            .get(chat_configuration_arn)
            .ok_or_else(|| ChatbotError::ConfigurationNotFound(chat_configuration_arn.to_string()))
    }

    // FIX(terraform-e2e): UpdateMicrosoftTeamsChannelConfiguration needed by terraform
    // provider to update Teams channel config in-place.
    #[allow(clippy::too_many_arguments)]
    pub fn update_microsoft_teams_channel_configuration(
        &mut self,
        chat_configuration_arn: &str,
        channel_id: &str,
        channel_name: Option<String>,
        iam_role_arn: Option<String>,
        sns_topic_arns: Option<Vec<String>>,
        logging_level: Option<String>,
        guardrail_policy_arns: Option<Vec<String>>,
        user_authorization_required: Option<bool>,
    ) -> Result<TeamsConfig, ChatbotError> {
        let config = self
            .teams_configs
            .get_mut(chat_configuration_arn)
            .ok_or_else(|| {
                ChatbotError::ConfigurationNotFound(chat_configuration_arn.to_string())
            })?;
        config.channel_id = channel_id.to_string();
        if let Some(name) = channel_name {
            config.channel_name = Some(name);
        }
        if let Some(iam) = iam_role_arn {
            config.iam_role_arn = iam;
        }
        if let Some(arns) = sns_topic_arns {
            config.sns_topic_arns = arns;
        }
        if let Some(level) = logging_level {
            config.logging_level = Some(level);
        }
        if let Some(arns) = guardrail_policy_arns {
            config.guardrail_policy_arns = arns;
        }
        if let Some(auth) = user_authorization_required {
            config.user_authorization_required = Some(auth);
        }
        Ok(config.clone())
    }

    // FIX(terraform-e2e): ListTagsForResource needed by terraform provider when tags
    // are specified on chatbot resources.
    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<HashMap<String, String>, ChatbotError> {
        if let Some(config) = self.slack_configs.get(resource_arn) {
            return Ok(config.tags.clone());
        }
        if let Some(config) = self.chime_configs.get(resource_arn) {
            return Ok(config.tags.clone());
        }
        if let Some(config) = self.teams_configs.get(resource_arn) {
            return Ok(config.tags.clone());
        }
        Err(ChatbotError::ResourceNotFound(resource_arn.to_string()))
    }

    // FIX(terraform-e2e): TagResource needed by terraform provider for tag management.
    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), ChatbotError> {
        if let Some(config) = self.slack_configs.get_mut(resource_arn) {
            config.tags.extend(tags);
            return Ok(());
        }
        if let Some(config) = self.chime_configs.get_mut(resource_arn) {
            config.tags.extend(tags);
            return Ok(());
        }
        if let Some(config) = self.teams_configs.get_mut(resource_arn) {
            config.tags.extend(tags);
            return Ok(());
        }
        Err(ChatbotError::ResourceNotFound(resource_arn.to_string()))
    }

    // FIX(terraform-e2e): UntagResource needed by terraform provider for tag management.
    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), ChatbotError> {
        if let Some(config) = self.slack_configs.get_mut(resource_arn) {
            for key in tag_keys {
                config.tags.remove(key);
            }
            return Ok(());
        }
        if let Some(config) = self.chime_configs.get_mut(resource_arn) {
            for key in tag_keys {
                config.tags.remove(key);
            }
            return Ok(());
        }
        if let Some(config) = self.teams_configs.get_mut(resource_arn) {
            for key in tag_keys {
                config.tags.remove(key);
            }
            return Ok(());
        }
        Err(ChatbotError::ResourceNotFound(resource_arn.to_string()))
    }
}

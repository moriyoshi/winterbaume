use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ChatbotService;
use crate::state::ChatbotState;
use crate::types::{ChimeConfig, SlackConfig, TeamsConfig};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatbotStateView {
    #[serde(default)]
    pub slack_configs: HashMap<String, SlackConfigView>,
    #[serde(default)]
    pub chime_configs: HashMap<String, ChimeConfigView>,
    #[serde(default)]
    pub teams_configs: HashMap<String, TeamsConfigView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackConfigView {
    pub arn: String,
    pub configuration_name: String,
    pub slack_team_id: String,
    pub slack_channel_id: String,
    pub slack_channel_name: Option<String>,
    pub iam_role_arn: String,
    pub sns_topic_arns: Vec<String>,
    pub logging_level: Option<String>,
    pub guardrail_policy_arns: Vec<String>,
    pub user_authorization_required: Option<bool>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChimeConfigView {
    pub arn: String,
    pub configuration_name: String,
    pub webhook_url: String,
    pub webhook_description: String,
    pub iam_role_arn: String,
    pub sns_topic_arns: Vec<String>,
    pub logging_level: Option<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsConfigView {
    pub arn: String,
    pub configuration_name: String,
    pub team_id: String,
    pub team_name: Option<String>,
    pub tenant_id: String,
    pub channel_id: String,
    pub channel_name: Option<String>,
    pub iam_role_arn: String,
    pub sns_topic_arns: Vec<String>,
    pub logging_level: Option<String>,
    pub guardrail_policy_arns: Vec<String>,
    pub user_authorization_required: Option<bool>,
    pub tags: HashMap<String, String>,
}

impl From<&SlackConfig> for SlackConfigView {
    fn from(c: &SlackConfig) -> Self {
        Self {
            arn: c.arn.clone(),
            configuration_name: c.configuration_name.clone(),
            slack_team_id: c.slack_team_id.clone(),
            slack_channel_id: c.slack_channel_id.clone(),
            slack_channel_name: c.slack_channel_name.clone(),
            iam_role_arn: c.iam_role_arn.clone(),
            sns_topic_arns: c.sns_topic_arns.clone(),
            logging_level: c.logging_level.clone(),
            guardrail_policy_arns: c.guardrail_policy_arns.clone(),
            user_authorization_required: c.user_authorization_required,
            tags: c.tags.clone(),
        }
    }
}

impl From<SlackConfigView> for SlackConfig {
    fn from(v: SlackConfigView) -> Self {
        Self {
            arn: v.arn,
            configuration_name: v.configuration_name,
            slack_team_id: v.slack_team_id,
            slack_channel_id: v.slack_channel_id,
            slack_channel_name: v.slack_channel_name,
            iam_role_arn: v.iam_role_arn,
            sns_topic_arns: v.sns_topic_arns,
            logging_level: v.logging_level,
            guardrail_policy_arns: v.guardrail_policy_arns,
            user_authorization_required: v.user_authorization_required,
            tags: v.tags,
        }
    }
}

impl From<&ChimeConfig> for ChimeConfigView {
    fn from(c: &ChimeConfig) -> Self {
        Self {
            arn: c.arn.clone(),
            configuration_name: c.configuration_name.clone(),
            webhook_url: c.webhook_url.clone(),
            webhook_description: c.webhook_description.clone(),
            iam_role_arn: c.iam_role_arn.clone(),
            sns_topic_arns: c.sns_topic_arns.clone(),
            logging_level: c.logging_level.clone(),
            tags: c.tags.clone(),
        }
    }
}

impl From<ChimeConfigView> for ChimeConfig {
    fn from(v: ChimeConfigView) -> Self {
        Self {
            arn: v.arn,
            configuration_name: v.configuration_name,
            webhook_url: v.webhook_url,
            webhook_description: v.webhook_description,
            iam_role_arn: v.iam_role_arn,
            sns_topic_arns: v.sns_topic_arns,
            logging_level: v.logging_level,
            tags: v.tags,
        }
    }
}

impl From<&TeamsConfig> for TeamsConfigView {
    fn from(c: &TeamsConfig) -> Self {
        Self {
            arn: c.arn.clone(),
            configuration_name: c.configuration_name.clone(),
            team_id: c.team_id.clone(),
            team_name: c.team_name.clone(),
            tenant_id: c.tenant_id.clone(),
            channel_id: c.channel_id.clone(),
            channel_name: c.channel_name.clone(),
            iam_role_arn: c.iam_role_arn.clone(),
            sns_topic_arns: c.sns_topic_arns.clone(),
            logging_level: c.logging_level.clone(),
            guardrail_policy_arns: c.guardrail_policy_arns.clone(),
            user_authorization_required: c.user_authorization_required,
            tags: c.tags.clone(),
        }
    }
}

impl From<TeamsConfigView> for TeamsConfig {
    fn from(v: TeamsConfigView) -> Self {
        Self {
            arn: v.arn,
            configuration_name: v.configuration_name,
            team_id: v.team_id,
            team_name: v.team_name,
            tenant_id: v.tenant_id,
            channel_id: v.channel_id,
            channel_name: v.channel_name,
            iam_role_arn: v.iam_role_arn,
            sns_topic_arns: v.sns_topic_arns,
            logging_level: v.logging_level,
            guardrail_policy_arns: v.guardrail_policy_arns,
            user_authorization_required: v.user_authorization_required,
            tags: v.tags,
        }
    }
}

impl From<&ChatbotState> for ChatbotStateView {
    fn from(state: &ChatbotState) -> Self {
        Self {
            slack_configs: state
                .slack_configs
                .iter()
                .map(|(k, v)| (k.clone(), SlackConfigView::from(v)))
                .collect(),
            chime_configs: state
                .chime_configs
                .iter()
                .map(|(k, v)| (k.clone(), ChimeConfigView::from(v)))
                .collect(),
            teams_configs: state
                .teams_configs
                .iter()
                .map(|(k, v)| (k.clone(), TeamsConfigView::from(v)))
                .collect(),
        }
    }
}

impl From<ChatbotStateView> for ChatbotState {
    fn from(view: ChatbotStateView) -> Self {
        Self {
            slack_configs: view
                .slack_configs
                .into_iter()
                .map(|(k, v)| (k, SlackConfig::from(v)))
                .collect(),
            chime_configs: view
                .chime_configs
                .into_iter()
                .map(|(k, v)| (k, ChimeConfig::from(v)))
                .collect(),
            teams_configs: view
                .teams_configs
                .into_iter()
                .map(|(k, v)| (k, TeamsConfig::from(v)))
                .collect(),
        }
    }
}

impl StatefulService for ChatbotService {
    type StateView = ChatbotStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ChatbotStateView::from(&*guard)
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
            *guard = ChatbotState::from(view);
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
            for (k, v) in view.slack_configs {
                guard.slack_configs.insert(k, SlackConfig::from(v));
            }
            for (k, v) in view.chime_configs {
                guard.chime_configs.insert(k, ChimeConfig::from(v));
            }
            for (k, v) in view.teams_configs {
                guard.teams_configs.insert(k, TeamsConfig::from(v));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

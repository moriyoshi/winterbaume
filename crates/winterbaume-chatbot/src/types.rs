use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SlackConfig {
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

#[derive(Debug, Clone)]
pub struct ChimeConfig {
    pub arn: String,
    pub configuration_name: String,
    pub webhook_url: String,
    pub webhook_description: String,
    pub iam_role_arn: String,
    pub sns_topic_arns: Vec<String>,
    pub logging_level: Option<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct TeamsConfig {
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

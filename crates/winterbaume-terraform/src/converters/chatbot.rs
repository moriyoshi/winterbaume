//! Terraform converters for AWS Chatbot resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_chatbot::ChatbotService;
use winterbaume_chatbot::views::{ChatbotStateView, SlackConfigView, TeamsConfigView};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_chatbot_slack_channel_configuration
// ---------------------------------------------------------------------------

pub struct AwsChatbotSlackChannelConfigurationConverter {
    service: Arc<ChatbotService>,
}

impl AwsChatbotSlackChannelConfigurationConverter {
    pub fn new(service: Arc<ChatbotService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsChatbotSlackChannelConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_chatbot_slack_channel_configuration"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsChatbotSlackChannelConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let configuration_name = require_str(
            attrs,
            "configuration_name",
            "aws_chatbot_slack_channel_configuration",
        )?;
        let slack_team_id = require_str(
            attrs,
            "slack_team_id",
            "aws_chatbot_slack_channel_configuration",
        )?;
        let slack_channel_id = require_str(
            attrs,
            "slack_channel_id",
            "aws_chatbot_slack_channel_configuration",
        )?;
        let iam_role_arn = require_str(
            attrs,
            "iam_role_arn",
            "aws_chatbot_slack_channel_configuration",
        )?;

        let arn = optional_str(attrs, "chat_configuration_arn")
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                format!(
                    "arn:aws:chatbot:{region}:{}:chat-configuration/slack-channel/{configuration_name}",
                    ctx.default_account_id
                )
            });

        let tags = extract_tags(attrs);

        let config_view = SlackConfigView {
            arn: arn.clone(),
            configuration_name: configuration_name.to_string(),
            slack_team_id: slack_team_id.to_string(),
            slack_channel_id: slack_channel_id.to_string(),
            slack_channel_name: optional_str(attrs, "slack_channel_name").map(|s| s.to_string()),
            iam_role_arn: iam_role_arn.to_string(),
            sns_topic_arns: vec![],
            logging_level: optional_str(attrs, "logging_level").map(|s| s.to_string()),
            guardrail_policy_arns: vec![],
            user_authorization_required: None,
            tags,
        };

        let mut state_view = ChatbotStateView::default();
        state_view.slack_configs.insert(arn, config_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for cfg in view.slack_configs.values() {
            let attrs = serde_json::json!({
                "chat_configuration_arn": cfg.arn,
                "configuration_name": cfg.configuration_name,
                "slack_team_id": cfg.slack_team_id,
                "slack_channel_id": cfg.slack_channel_id,
                "slack_channel_name": cfg.slack_channel_name,
                "iam_role_arn": cfg.iam_role_arn,
                "logging_level": cfg.logging_level,
                "tags": cfg.tags,
            });
            results.push(ExtractedResource {
                name: cfg.configuration_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_chatbot_teams_channel_configuration
// ---------------------------------------------------------------------------

pub struct AwsChatbotTeamsChannelConfigurationConverter {
    service: Arc<ChatbotService>,
}

impl AwsChatbotTeamsChannelConfigurationConverter {
    pub fn new(service: Arc<ChatbotService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsChatbotTeamsChannelConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_chatbot_microsoft_teams_channel_configuration"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsChatbotTeamsChannelConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let configuration_name = require_str(
            attrs,
            "configuration_name",
            "aws_chatbot_microsoft_teams_channel_configuration",
        )?;
        let team_id = require_str(
            attrs,
            "team_id",
            "aws_chatbot_microsoft_teams_channel_configuration",
        )?;
        let tenant_id = require_str(
            attrs,
            "tenant_id",
            "aws_chatbot_microsoft_teams_channel_configuration",
        )?;
        let channel_id = require_str(
            attrs,
            "channel_id",
            "aws_chatbot_microsoft_teams_channel_configuration",
        )?;
        let iam_role_arn = require_str(
            attrs,
            "iam_role_arn",
            "aws_chatbot_microsoft_teams_channel_configuration",
        )?;

        let arn = optional_str(attrs, "chat_configuration_arn")
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                format!(
                    "arn:aws:chatbot:{region}:{}:chat-configuration/microsoft-teams-channel/{configuration_name}",
                    ctx.default_account_id
                )
            });

        let tags = extract_tags(attrs);

        let config_view = TeamsConfigView {
            arn: arn.clone(),
            configuration_name: configuration_name.to_string(),
            team_id: team_id.to_string(),
            team_name: optional_str(attrs, "team_name").map(|s| s.to_string()),
            tenant_id: tenant_id.to_string(),
            channel_id: channel_id.to_string(),
            channel_name: optional_str(attrs, "channel_name").map(|s| s.to_string()),
            iam_role_arn: iam_role_arn.to_string(),
            sns_topic_arns: vec![],
            logging_level: optional_str(attrs, "logging_level").map(|s| s.to_string()),
            guardrail_policy_arns: vec![],
            user_authorization_required: None,
            tags,
        };

        let mut state_view = ChatbotStateView::default();
        state_view.teams_configs.insert(arn, config_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for cfg in view.teams_configs.values() {
            let attrs = serde_json::json!({
                "chat_configuration_arn": cfg.arn,
                "configuration_name": cfg.configuration_name,
                "team_id": cfg.team_id,
                "team_name": cfg.team_name,
                "tenant_id": cfg.tenant_id,
                "channel_id": cfg.channel_id,
                "channel_name": cfg.channel_name,
                "iam_role_arn": cfg.iam_role_arn,
                "logging_level": cfg.logging_level,
                "tags": cfg.tags,
            });
            results.push(ExtractedResource {
                name: cfg.configuration_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn extract_tags(attrs: &serde_json::Value) -> HashMap<String, String> {
    let mut tags = HashMap::new();
    if let Some(obj) = attrs.get("tags").and_then(|v| v.as_object()) {
        for (k, v) in obj {
            if let Some(s) = v.as_str() {
                tags.insert(k.clone(), s.to_string());
            }
        }
    }
    tags
}

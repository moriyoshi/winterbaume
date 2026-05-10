//! Terraform converters for AWS Chatbot resources.
//!
//! `SlackConfigTfModel` and `TeamsConfigTfModel` are generated from
//! `specs/chatbot.toml`. The chat-configuration ARN template, the
//! constant-empty `sns_topic_arns` / `guardrail_policy_arns` lists, and
//! the `user_authorization_required = None` default are wired up here.

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
use crate::generated::chatbot as chatbot_gen;
use crate::util::{classify_deserialize_error, extract_region};

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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: chatbot_gen::SlackConfigTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_chatbot_slack_channel_configuration", e)
            })?;

        let arn = model.chat_configuration_arn.unwrap_or_else(|| {
            format!(
                "arn:aws:chatbot:{region}:{}:chat-configuration/slack-channel/{}",
                ctx.default_account_id, model.configuration_name
            )
        });

        let config_view = SlackConfigView {
            arn: arn.clone(),
            configuration_name: model.configuration_name,
            slack_team_id: model.slack_team_id,
            slack_channel_id: model.slack_channel_id,
            slack_channel_name: model.slack_channel_name,
            iam_role_arn: model.iam_role_arn,
            sns_topic_arns: vec![],
            logging_level: model.logging_level,
            guardrail_policy_arns: vec![],
            user_authorization_required: None,
            tags: model.tags,
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
// aws_chatbot_microsoft_teams_channel_configuration
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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: chatbot_gen::TeamsConfigTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_chatbot_microsoft_teams_channel_configuration", e)
            })?;

        let arn = model.chat_configuration_arn.unwrap_or_else(|| {
            format!(
                "arn:aws:chatbot:{region}:{}:chat-configuration/microsoft-teams-channel/{}",
                ctx.default_account_id, model.configuration_name
            )
        });

        let config_view = TeamsConfigView {
            arn: arn.clone(),
            configuration_name: model.configuration_name,
            team_id: model.team_id,
            team_name: model.team_name,
            tenant_id: model.tenant_id,
            channel_id: model.channel_id,
            channel_name: model.channel_name,
            iam_role_arn: model.iam_role_arn,
            sns_topic_arns: vec![],
            logging_level: model.logging_level,
            guardrail_policy_arns: vec![],
            user_authorization_required: None,
            tags: model.tags,
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

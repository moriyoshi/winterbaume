//! Terraform converter for Lex Models V2 resources.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_lexmodelsv2::LexModelsV2Service;
use winterbaume_lexmodelsv2::views::{BotView, LexStateView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_lexv2models_bot
// ---------------------------------------------------------------------------

pub struct AwsLexv2modelsBotConverter {
    service: Arc<LexModelsV2Service>,
}

impl AwsLexv2modelsBotConverter {
    pub fn new(service: Arc<LexModelsV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLexv2modelsBotConverter {
    fn resource_type(&self) -> &str {
        "aws_lexv2models_bot"
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

impl AwsLexv2modelsBotConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let bot_id = optional_str(attrs, "id")
            .unwrap_or_else(|| format!("bot-{}", uuid::Uuid::new_v4().simple()));
        let name = require_str(attrs, "name", "aws_lexv2models_bot")?;
        let role_arn = optional_str(attrs, "role_arn").unwrap_or_default();
        let description = optional_str(attrs, "description");
        let idle_session_ttl_in_seconds = attrs
            .get("idle_session_ttl_in_seconds")
            .and_then(|v| v.as_i64())
            .unwrap_or(300) as i32;
        let _bot_type = optional_str(attrs, "type");
        let _test_bot_alias_tags = attrs.get("test_bot_alias_tags");
        let _members = attrs.get("members");
        let _timeouts = attrs.get("timeouts");

        // TF schema nests child_directed inside a `data_privacy` block.
        let data_privacy_child_directed = attrs
            .get("data_privacy")
            .and_then(|dp| dp.as_array())
            .and_then(|arr| arr.first())
            .and_then(|block| block.get("child_directed"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let tags = extract_tags(attrs);

        let bot_view = BotView {
            bot_id: bot_id.clone(),
            bot_name: name.to_string(),
            role_arn,
            data_privacy_child_directed,
            idle_session_ttl_in_seconds,
            bot_status: "Available".to_string(),
            description,
            creation_date_time: attrs
                .get("create_time")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            last_updated_date_time: attrs
                .get("create_time")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            tags,
        };

        let mut state_view = LexStateView::default();
        state_view.bots.insert(bot_view.bot_id.clone(), bot_view);
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
        for bot in view.bots.values() {
            let arn = format!(
                "arn:aws:lex:{}:{}:bot/{}",
                ctx.default_region, ctx.default_account_id, bot.bot_id,
            );
            let attrs = serde_json::json!({
                "id": bot.bot_id,
                "name": bot.bot_name,
                "arn": arn,
                "role_arn": bot.role_arn,
                "data_privacy": [{ "child_directed": bot.data_privacy_child_directed }],
                "idle_session_ttl_in_seconds": bot.idle_session_ttl_in_seconds,
                "description": bot.description,
                "type": "Bot",
                "tags": bot.tags,
                "tags_all": bot.tags,
                "test_bot_alias_tags": {},
                "members": [],
            });
            results.push(ExtractedResource {
                name: bot.bot_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

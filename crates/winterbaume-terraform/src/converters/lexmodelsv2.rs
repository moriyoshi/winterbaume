//! Terraform converter for Lex Models V2 resources.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_lexmodelsv2::LexModelsV2Service;
use winterbaume_lexmodelsv2::views::{
    BotLocaleView, BotVersionView, BotView, IntentView, LexStateView, SlotTypeView, SlotView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::lexmodelsv2 as lexmodelsv2_gen;
use crate::util::{classify_deserialize_error, extract_region};

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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: lexmodelsv2_gen::BotTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_lexv2models_bot", e))?;

        let bot_id = model
            .id
            .unwrap_or_else(|| format!("bot-{}", uuid::Uuid::new_v4().simple()));
        let role_arn = model.role_arn.unwrap_or_default();
        let idle_session_ttl_in_seconds = model.idle_session_ttl_in_seconds as i32;

        // TF schema nests child_directed inside a `data_privacy` block — parse
        // straight from raw attributes since the spec can't model array-of-object.
        let data_privacy_child_directed = instance
            .attributes
            .get("data_privacy")
            .and_then(|dp| dp.as_array())
            .and_then(|arr| arr.first())
            .and_then(|block| block.get("child_directed"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let create_time = instance
            .attributes
            .get("create_time")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let bot_view = BotView {
            bot_id: bot_id.clone(),
            bot_name: model.name,
            role_arn,
            data_privacy_child_directed,
            idle_session_ttl_in_seconds,
            bot_status: "Available".to_string(),
            description: model.description,
            creation_date_time: create_time.clone(),
            last_updated_date_time: create_time,
            tags: model.tags,
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

// ---------------------------------------------------------------------------
// aws_lexv2models_bot_locale
// ---------------------------------------------------------------------------

/// Converts `aws_lexv2models_bot_locale` Terraform resources to/from Lex state.
pub struct AwsLexv2modelsBotLocaleConverter {
    service: Arc<LexModelsV2Service>,
}

impl AwsLexv2modelsBotLocaleConverter {
    pub fn new(service: Arc<LexModelsV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLexv2modelsBotLocaleConverter {
    fn resource_type(&self) -> &str {
        "aws_lexv2models_bot_locale"
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

impl AwsLexv2modelsBotLocaleConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: lexmodelsv2_gen::BotLocaleTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_lexv2models_bot_locale", e))?;

        let nlu_intent_confidence_threshold = instance
            .attributes
            .get("n_lu_intent_confidence_threshold")
            .or_else(|| instance.attributes.get("nlu_intent_confidence_threshold"))
            .and_then(|v| v.as_f64())
            .unwrap_or(0.40);

        let create_time = instance
            .attributes
            .get("creation_date_time")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let locale_name = model.locale_name.unwrap_or_else(|| model.locale_id.clone());

        let bot_locale_view = BotLocaleView {
            bot_id: model.bot_id.clone(),
            locale_id: model.locale_id.clone(),
            locale_name,
            bot_version: model.bot_version.clone(),
            nlu_intent_confidence_threshold,
            bot_locale_status: "Built".to_string(),
            description: model.description,
            creation_date_time: create_time.clone(),
            last_updated_date_time: create_time,
        };

        let mut state_view = LexStateView::default();
        let key = format!("{}/{}", model.bot_id, model.locale_id);
        state_view.bot_locales.insert(key, bot_locale_view);
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
        for bl in view.bot_locales.values() {
            let id = format!("{},{},{}", bl.bot_id, bl.bot_version, bl.locale_id);
            let attrs = serde_json::json!({
                "id": id,
                "bot_id": bl.bot_id,
                "bot_version": bl.bot_version,
                "locale_id": bl.locale_id,
                "name": bl.locale_name,
                "description": bl.description,
                "n_lu_intent_confidence_threshold": bl.nlu_intent_confidence_threshold,
            });
            results.push(ExtractedResource {
                name: bl.locale_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_lexv2models_bot_version
// ---------------------------------------------------------------------------

/// Converts `aws_lexv2models_bot_version` Terraform resources to/from Lex state.
pub struct AwsLexv2modelsBotVersionConverter {
    service: Arc<LexModelsV2Service>,
}

impl AwsLexv2modelsBotVersionConverter {
    pub fn new(service: Arc<LexModelsV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLexv2modelsBotVersionConverter {
    fn resource_type(&self) -> &str {
        "aws_lexv2models_bot_version"
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

impl AwsLexv2modelsBotVersionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: lexmodelsv2_gen::BotVersionTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_lexv2models_bot_version", e))?;

        // Terraform may have already populated `bot_version` after a create;
        // otherwise default to "1" (the first numeric version Lex assigns).
        let bot_version = instance
            .attributes
            .get("bot_version")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "1".to_string());

        let create_time = instance
            .attributes
            .get("creation_date_time")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let bot_version_view = BotVersionView {
            bot_id: model.bot_id.clone(),
            bot_version: bot_version.clone(),
            bot_status: "Available".to_string(),
            description: model.description,
            creation_date_time: create_time,
        };

        let mut state_view = LexStateView::default();
        let key = format!("{}/{}", model.bot_id, bot_version);
        state_view.bot_versions.insert(key, bot_version_view);
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
        for bv in view.bot_versions.values() {
            let id = format!("{},{}", bv.bot_id, bv.bot_version);
            let attrs = serde_json::json!({
                "id": id,
                "bot_id": bv.bot_id,
                "bot_version": bv.bot_version,
                "description": bv.description,
            });
            results.push(ExtractedResource {
                name: bv.bot_version.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_lexv2models_intent
// ---------------------------------------------------------------------------

/// Converts `aws_lexv2models_intent` Terraform resources to/from Lex state.
pub struct AwsLexv2modelsIntentConverter {
    service: Arc<LexModelsV2Service>,
}

impl AwsLexv2modelsIntentConverter {
    pub fn new(service: Arc<LexModelsV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLexv2modelsIntentConverter {
    fn resource_type(&self) -> &str {
        "aws_lexv2models_intent"
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

impl AwsLexv2modelsIntentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: lexmodelsv2_gen::IntentTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_lexv2models_intent", e))?;

        let intent_id = model
            .intent_id
            .unwrap_or_else(|| format!("intent-{}", uuid::Uuid::new_v4().simple()));

        let create_time = instance
            .attributes
            .get("creation_date_time")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let intent_view = IntentView {
            intent_id: intent_id.clone(),
            intent_name: model.intent_name,
            bot_id: model.bot_id.clone(),
            bot_version: model.bot_version.clone(),
            locale_id: model.locale_id.clone(),
            description: model.description,
            creation_date_time: create_time.clone(),
            last_updated_date_time: create_time,
        };

        let mut state_view = LexStateView::default();
        let key = format!("{}/{}/{}", model.bot_id, model.locale_id, intent_id);
        state_view.intents.insert(key, intent_view);
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
        for intent in view.intents.values() {
            let id = format!(
                "{},{},{},{}",
                intent.bot_id, intent.bot_version, intent.locale_id, intent.intent_id
            );
            let attrs = serde_json::json!({
                "id": id,
                "intent_id": intent.intent_id,
                "name": intent.intent_name,
                "bot_id": intent.bot_id,
                "bot_version": intent.bot_version,
                "locale_id": intent.locale_id,
                "description": intent.description,
            });
            results.push(ExtractedResource {
                name: intent.intent_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_lexv2models_slot
// ---------------------------------------------------------------------------

/// Converts `aws_lexv2models_slot` Terraform resources to/from Lex state.
pub struct AwsLexv2modelsSlotConverter {
    service: Arc<LexModelsV2Service>,
}

impl AwsLexv2modelsSlotConverter {
    pub fn new(service: Arc<LexModelsV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLexv2modelsSlotConverter {
    fn resource_type(&self) -> &str {
        "aws_lexv2models_slot"
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

impl AwsLexv2modelsSlotConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: lexmodelsv2_gen::SlotTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_lexv2models_slot", e))?;

        let slot_id = model
            .slot_id
            .unwrap_or_else(|| format!("slot-{}", uuid::Uuid::new_v4().simple()));

        let create_time = instance
            .attributes
            .get("creation_date_time")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        // Nested blocks stay as opaque JSON; fall back to Null when absent.
        let value_elicitation_setting = instance
            .attributes
            .get("value_elicitation_setting")
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        let multiple_values_setting = instance.attributes.get("multiple_values_setting").cloned();
        let obfuscation_setting = instance.attributes.get("obfuscation_setting").cloned();
        let sub_slot_setting = instance.attributes.get("sub_slot_setting").cloned();

        let slot_view = SlotView {
            slot_id: slot_id.clone(),
            slot_name: model.slot_name,
            bot_id: model.bot_id.clone(),
            bot_version: model.bot_version.clone(),
            locale_id: model.locale_id.clone(),
            intent_id: model.intent_id.clone(),
            slot_type_id: model.slot_type_id,
            description: model.description,
            creation_date_time: create_time.clone(),
            last_updated_date_time: create_time,
            value_elicitation_setting,
            multiple_values_setting,
            obfuscation_setting,
            sub_slot_setting,
        };

        let mut state_view = LexStateView::default();
        let key = format!(
            "{}/{}/{}/{}",
            model.bot_id, model.locale_id, model.intent_id, slot_id
        );
        state_view.slots.insert(key, slot_view);
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
        for slot in view.slots.values() {
            let id = format!(
                "{},{},{},{},{}",
                slot.bot_id, slot.bot_version, slot.intent_id, slot.locale_id, slot.slot_id
            );
            let attrs = serde_json::json!({
                "id": id,
                "slot_id": slot.slot_id,
                "name": slot.slot_name,
                "bot_id": slot.bot_id,
                "bot_version": slot.bot_version,
                "locale_id": slot.locale_id,
                "intent_id": slot.intent_id,
                "slot_type_id": slot.slot_type_id,
                "description": slot.description,
            });
            results.push(ExtractedResource {
                name: slot.slot_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_lexv2models_slot_type
// ---------------------------------------------------------------------------

/// Converts `aws_lexv2models_slot_type` Terraform resources to/from Lex state.
pub struct AwsLexv2modelsSlotTypeConverter {
    service: Arc<LexModelsV2Service>,
}

impl AwsLexv2modelsSlotTypeConverter {
    pub fn new(service: Arc<LexModelsV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLexv2modelsSlotTypeConverter {
    fn resource_type(&self) -> &str {
        "aws_lexv2models_slot_type"
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

impl AwsLexv2modelsSlotTypeConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: lexmodelsv2_gen::SlotTypeTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_lexv2models_slot_type", e))?;

        let slot_type_id = model
            .slot_type_id
            .unwrap_or_else(|| format!("slottype-{}", uuid::Uuid::new_v4().simple()));

        let create_time = instance
            .attributes
            .get("creation_date_time")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        // Nested blocks stay as opaque JSON; absent values become None.
        let slot_type_values = instance.attributes.get("slot_type_values").cloned();
        let value_selection_setting = instance.attributes.get("value_selection_setting").cloned();
        let external_source_setting = instance.attributes.get("external_source_setting").cloned();
        let composite_slot_type_setting = instance
            .attributes
            .get("composite_slot_type_setting")
            .cloned();

        let slot_type_view = SlotTypeView {
            slot_type_id: slot_type_id.clone(),
            slot_type_name: model.slot_type_name,
            bot_id: model.bot_id.clone(),
            bot_version: model.bot_version.clone(),
            locale_id: model.locale_id.clone(),
            description: model.description,
            parent_slot_type_signature: model.parent_slot_type_signature,
            slot_type_values,
            value_selection_setting,
            external_source_setting,
            composite_slot_type_setting,
            creation_date_time: create_time.clone(),
            last_updated_date_time: create_time,
        };

        let mut state_view = LexStateView::default();
        let key = format!("{}/{}/{}", model.bot_id, model.locale_id, slot_type_id);
        state_view.slot_types.insert(key, slot_type_view);
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
        for st in view.slot_types.values() {
            let id = format!(
                "{},{},{},{}",
                st.bot_id, st.bot_version, st.locale_id, st.slot_type_id
            );
            let attrs = serde_json::json!({
                "id": id,
                "slot_type_id": st.slot_type_id,
                "name": st.slot_type_name,
                "bot_id": st.bot_id,
                "bot_version": st.bot_version,
                "locale_id": st.locale_id,
                "description": st.description,
                "parent_slot_type_signature": st.parent_slot_type_signature,
            });
            results.push(ExtractedResource {
                name: st.slot_type_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

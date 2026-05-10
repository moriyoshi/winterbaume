//! Terraform converters for Pinpoint resources.
//!
//! `PinpointAppTfModel` and `EmailChannelTfModel` are generated from
//! `specs/pinpoint.toml`. The ARN templates, the synthesised
//! `application_id` (when absent), the merged `tags_all` overlay, and
//! the nested `campaign_hook` / `limits` / `quiet_time` blocks are
//! wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_pinpoint::PinpointService;
use winterbaume_pinpoint::views::{EmailChannelView, PinpointAppView, PinpointStateView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::pinpoint as pinpoint_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_pinpoint_app
// ---------------------------------------------------------------------------

/// Converts `aws_pinpoint_app` Terraform resources to/from Pinpoint state.
pub struct AwsPinpointAppConverter {
    service: Arc<PinpointService>,
}

impl AwsPinpointAppConverter {
    pub fn new(service: Arc<PinpointService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsPinpointAppConverter {
    fn resource_type(&self) -> &str {
        "aws_pinpoint_app"
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

impl AwsPinpointAppConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: pinpoint_gen::PinpointAppTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_pinpoint_app", e))?;

        let name = model.name.unwrap_or_default();
        let id = model
            .application_id
            .or(model.id)
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string().replace('-', ""));
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:mobiletargeting:{}:{}:apps/{}",
                region, ctx.default_account_id, id
            )
        });
        let creation_date = model
            .creation_date
            .unwrap_or_else(|| chrono::Utc::now().to_rfc3339());
        let mut tags = model.tags;
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags.entry(k.clone()).or_insert_with(|| s.to_string());
                }
            }
        }
        // Parse `campaign_hook` nested block: [{lambda_function_name: ..., mode: ..., web_url: ...}]
        let campaign_hook_view: Option<winterbaume_pinpoint::views::CampaignHookView> = attrs
            .get("campaign_hook")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .map(|b| winterbaume_pinpoint::views::CampaignHookView {
                lambda_function_name: b
                    .get("lambda_function_name")
                    .and_then(|v| v.as_str())
                    .map(String::from),
                mode: b.get("mode").and_then(|v| v.as_str()).map(String::from),
                web_url: b.get("web_url").and_then(|v| v.as_str()).map(String::from),
            });

        // Parse `limits` nested block: [{daily: ..., maximum_duration: ..., messages_per_second: ..., total: ..., session: ...}]
        let limits_view: Option<winterbaume_pinpoint::views::LimitsView> = attrs
            .get("limits")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .map(|b| winterbaume_pinpoint::views::LimitsView {
                daily: b.get("daily").and_then(|v| v.as_i64()),
                maximum_duration: b.get("maximum_duration").and_then(|v| v.as_i64()),
                messages_per_second: b.get("messages_per_second").and_then(|v| v.as_i64()),
                total: b.get("total").and_then(|v| v.as_i64()),
                session: b.get("session").and_then(|v| v.as_i64()),
            });

        // Parse `quiet_time` nested block: [{start: ..., end: ...}] — stored as extra JSON in settings
        let quiet_time: Option<serde_json::Value> = attrs
            .get("quiet_time")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .map(|b| {
                serde_json::json!({
                    "start": b.get("start").and_then(|v| v.as_str()).unwrap_or(""),
                    "end": b.get("end").and_then(|v| v.as_str()).unwrap_or(""),
                })
            });

        let settings = if campaign_hook_view.is_some() || limits_view.is_some() {
            Some(winterbaume_pinpoint::views::ApplicationSettingsView {
                campaign_hook: campaign_hook_view,
                limits: limits_view,
                last_modified_date: chrono::Utc::now().to_rfc3339(),
            })
        } else {
            None
        };

        let app_view = PinpointAppView {
            id: id.clone(),
            arn,
            name,
            creation_date,
            tags,
            settings,
            event_stream: None,
            quiet_time,
        };

        let mut state_view = PinpointStateView::default();
        state_view.apps.insert(id, app_view);
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
        for app in view.apps.values() {
            let campaign_hook_block: Vec<serde_json::Value> = app
                .settings
                .as_ref()
                .and_then(|s| s.campaign_hook.as_ref())
                .map(|h| {
                    vec![serde_json::json!({
                        "lambda_function_name": h.lambda_function_name,
                        "mode": h.mode,
                        "web_url": h.web_url,
                    })]
                })
                .unwrap_or_default();

            let limits_block: Vec<serde_json::Value> = app
                .settings
                .as_ref()
                .and_then(|s| s.limits.as_ref())
                .map(|l| {
                    vec![serde_json::json!({
                        "daily": l.daily,
                        "maximum_duration": l.maximum_duration,
                        "messages_per_second": l.messages_per_second,
                        "total": l.total,
                        "session": l.session,
                    })]
                })
                .unwrap_or_default();

            let quiet_time_block: Vec<serde_json::Value> = app
                .quiet_time
                .as_ref()
                .map(|qt| vec![qt.clone()])
                .unwrap_or_default();

            let attrs = serde_json::json!({
                "id": app.id,
                "application_id": app.id,
                "arn": app.arn,
                "name": app.name,
                "creation_date": app.creation_date,
                "tags": app.tags,
                "tags_all": app.tags,
                "campaign_hook": campaign_hook_block,
                "limits": limits_block,
                "quiet_time": quiet_time_block,
            });
            results.push(ExtractedResource {
                name: app.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_pinpoint_email_channel
// ---------------------------------------------------------------------------

/// Converts `aws_pinpoint_email_channel` Terraform resources to/from Pinpoint state.
pub struct AwsPinpointEmailChannelConverter {
    service: Arc<PinpointService>,
}

impl AwsPinpointEmailChannelConverter {
    pub fn new(service: Arc<PinpointService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsPinpointEmailChannelConverter {
    fn resource_type(&self) -> &str {
        "aws_pinpoint_email_channel"
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

impl AwsPinpointEmailChannelConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: pinpoint_gen::EmailChannelTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_pinpoint_email_channel", e))?;

        let application_id = model.application_id;
        let messages_per_second = attrs
            .get("messages_per_second")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);

        // Ensure the parent app exists in the state.
        let existing = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        if !existing.apps.contains_key(&application_id) {
            let app_view = PinpointAppView {
                id: application_id.clone(),
                arn: format!(
                    "arn:aws:mobiletargeting:{}:{}:apps/{}",
                    region, ctx.default_account_id, application_id
                ),
                name: String::new(),
                creation_date: chrono::Utc::now().to_rfc3339(),
                tags: HashMap::new(),
                settings: None,
                event_stream: None,
                quiet_time: None,
            };
            let mut app_state = PinpointStateView::default();
            app_state.apps.insert(application_id.clone(), app_view);
            self.service
                .merge(&ctx.default_account_id, &region, app_state)
                .await?;
        }

        let ec_view = EmailChannelView {
            application_id: application_id.clone(),
            enabled: model.enabled,
            from_address: model.from_address.unwrap_or_default(),
            identity: model.identity.unwrap_or_default(),
            role_arn: model.role_arn,
            configuration_set: model.configuration_set,
            messages_per_second,
        };

        let mut state_view = PinpointStateView::default();
        state_view.email_channels.insert(application_id, ec_view);
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
        for ec in view.email_channels.values() {
            let attrs = serde_json::json!({
                "id": ec.application_id,
                "application_id": ec.application_id,
                "enabled": ec.enabled,
                "from_address": ec.from_address,
                "identity": ec.identity,
                "role_arn": ec.role_arn,
                "configuration_set": ec.configuration_set,
                "messages_per_second": ec.messages_per_second,
            });
            results.push(ExtractedResource {
                name: ec.application_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

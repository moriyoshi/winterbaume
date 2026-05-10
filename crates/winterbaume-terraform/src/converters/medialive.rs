//! Terraform converters for MediaLive resources.
//!
//! `MediaLiveChannelTfModel` and `MediaLiveInputTfModel` are generated
//! from `specs/medialive.toml`. The ARN templates, the
//! `channel_id` / `input_id` / `id` fallback chain, the constants
//! (`SINGLE_PIPELINE`, `DISABLED`, `IDLE`, `DETACHED`, `STATIC`,
//! `UDP_PUSH`), the `pipelines_running_count` (i32), and the
//! nested-block / array fields (`input_attachments`, `destinations`,
//! `encoder_settings`, `input_specification`, `cdi_input_specification`,
//! `maintenance`, `vpc`, `attached_channels`, `input_devices`,
//! `media_connect_flows`, `security_groups`, `sources`) are wired up
//! here.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_medialive::MediaLiveService;
use winterbaume_medialive::views::{MediaLiveChannelView, MediaLiveInputView, MediaLiveStateView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::medialive as medialive_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_medialive_channel
// ---------------------------------------------------------------------------

pub struct AwsMedialiveChannelConverter {
    service: Arc<MediaLiveService>,
}

impl AwsMedialiveChannelConverter {
    pub fn new(service: Arc<MediaLiveService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMedialiveChannelConverter {
    fn resource_type(&self) -> &str {
        "aws_medialive_channel"
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

impl AwsMedialiveChannelConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: medialive_gen::MediaLiveChannelTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_medialive_channel", e))?;

        // Original converter requires `channel_id` first, falling back to `id`.
        let id =
            model
                .channel_id
                .or(model.id)
                .ok_or_else(|| ConversionError::MissingAttribute {
                    resource_type: "aws_medialive_channel".to_string(),
                    attribute: "channel_id".to_string(),
                })?;
        let name = model.name;
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:medialive:{}:{}:channel:{}",
                region, ctx.default_account_id, id
            )
        });
        let channel_class = model
            .channel_class
            .unwrap_or_else(|| "SINGLE_PIPELINE".to_string());
        let role_arn = model.role_arn.unwrap_or_default();
        let log_level = model.log_level.unwrap_or_else(|| "DISABLED".to_string());

        let attrs = &instance.attributes;
        let pipelines_running_count = attrs
            .get("pipelines_running_count")
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i32;

        let input_attachments = attrs
            .get("input_attachments")
            .cloned()
            .unwrap_or(serde_json::Value::Array(vec![]));
        let destinations = attrs
            .get("destinations")
            .cloned()
            .unwrap_or(serde_json::Value::Array(vec![]));
        let encoder_settings = attrs
            .get("encoder_settings")
            .cloned()
            .unwrap_or(serde_json::Value::Object(Default::default()));
        let input_specification = attrs
            .get("input_specification")
            .cloned()
            .unwrap_or(serde_json::Value::Object(Default::default()));
        let cdi_input_specification = attrs
            .get("cdi_input_specification")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });
        let maintenance = attrs
            .get("maintenance")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });
        let channel_vpc = attrs
            .get("vpc")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });

        let ch_view = MediaLiveChannelView {
            id,
            arn,
            name,
            state: "IDLE".to_string(),
            channel_class,
            pipelines_running_count,
            role_arn,
            input_attachments,
            destinations,
            encoder_settings,
            input_specification,
            log_level,
            tags: model.tags,
            cdi_input_specification,
            maintenance,
            vpc: channel_vpc,
        };

        let mut state_view = MediaLiveStateView::default();
        state_view.channels.push(ch_view);
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
        for ch in &view.channels {
            let attrs = serde_json::json!({
                "id": ch.id,
                "channel_id": ch.id,
                "arn": ch.arn,
                "name": ch.name,
                "state": ch.state,
                "channel_class": ch.channel_class,
                "pipelines_running_count": ch.pipelines_running_count,
                "role_arn": ch.role_arn,
                "input_attachments": ch.input_attachments,
                "destinations": ch.destinations,
                "encoder_settings": ch.encoder_settings,
                "input_specification": ch.input_specification,
                "log_level": ch.log_level,
                "tags": ch.tags,
                "cdi_input_specification": ch.cdi_input_specification,
                "maintenance": ch.maintenance,
                "vpc": ch.vpc,
            });
            results.push(ExtractedResource {
                name: ch.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_medialive_input
// ---------------------------------------------------------------------------

pub struct AwsMedialiveInputConverter {
    service: Arc<MediaLiveService>,
}

impl AwsMedialiveInputConverter {
    pub fn new(service: Arc<MediaLiveService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMedialiveInputConverter {
    fn resource_type(&self) -> &str {
        "aws_medialive_input"
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

impl AwsMedialiveInputConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: medialive_gen::MediaLiveInputTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_medialive_input", e))?;

        let id = model
            .input_id
            .or(model.id)
            .ok_or_else(|| ConversionError::MissingAttribute {
                resource_type: "aws_medialive_input".to_string(),
                attribute: "input_id".to_string(),
            })?;
        let name = model.name;
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:medialive:{}:{}:input:{}",
                region, ctx.default_account_id, id
            )
        });
        let input_class = model
            .input_class
            .unwrap_or_else(|| "SINGLE_PIPELINE".to_string());
        let input_source_type = model
            .input_source_type
            .unwrap_or_else(|| "STATIC".to_string());
        let input_type = model.input_type.unwrap_or_else(|| "UDP_PUSH".to_string());
        let role_arn = model.role_arn.unwrap_or_default();

        let attrs = &instance.attributes;
        let attached_channels: Vec<String> = attrs
            .get("attached_channels")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();
        let destinations = attrs
            .get("destinations")
            .cloned()
            .unwrap_or(serde_json::Value::Array(vec![]));
        let input_devices = attrs
            .get("input_devices")
            .cloned()
            .unwrap_or(serde_json::Value::Array(vec![]));
        let media_connect_flows = attrs
            .get("media_connect_flows")
            .cloned()
            .unwrap_or(serde_json::Value::Array(vec![]));
        let security_groups: Vec<String> = attrs
            .get("security_groups")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();
        let sources = attrs
            .get("sources")
            .cloned()
            .unwrap_or(serde_json::Value::Array(vec![]));
        let input_vpc = attrs
            .get("vpc")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });

        let inp_view = MediaLiveInputView {
            id,
            arn,
            name,
            state: "DETACHED".to_string(),
            input_class,
            input_source_type,
            input_type,
            role_arn,
            attached_channels,
            destinations,
            input_devices,
            media_connect_flows,
            security_groups,
            sources,
            tags: model.tags,
            vpc: input_vpc,
        };

        let mut state_view = MediaLiveStateView::default();
        state_view.inputs.push(inp_view);
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
        for inp in &view.inputs {
            let attrs = serde_json::json!({
                "id": inp.id,
                "input_id": inp.id,
                "arn": inp.arn,
                "name": inp.name,
                "state": inp.state,
                "input_class": inp.input_class,
                "input_source_type": inp.input_source_type,
                "type": inp.input_type,
                "role_arn": inp.role_arn,
                "attached_channels": inp.attached_channels,
                "destinations": inp.destinations,
                "input_devices": inp.input_devices,
                "media_connect_flows": inp.media_connect_flows,
                "security_groups": inp.security_groups,
                "sources": inp.sources,
                "tags": inp.tags,
                "vpc": inp.vpc,
            });
            results.push(ExtractedResource {
                name: inp.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

//! Terraform converters for AWS Config resources.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_config::ConfigService;
use winterbaume_config::views::{
    ConfigRuleView, ConfigStateView, ConfigurationRecorderView, DeliveryChannelView,
    RecordingGroupView,
};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_config_configuration_recorder
// ---------------------------------------------------------------------------

pub struct AwsConfigConfigurationRecorderConverter {
    service: Arc<ConfigService>,
}

impl AwsConfigConfigurationRecorderConverter {
    pub fn new(service: Arc<ConfigService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsConfigConfigurationRecorderConverter {
    fn resource_type(&self) -> &str {
        "aws_config_configuration_recorder"
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

impl AwsConfigConfigurationRecorderConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = optional_str(attrs, "name").unwrap_or_else(|| "default".to_string());
        let role_arn = optional_str(attrs, "role_arn").unwrap_or_default();

        let all_supported = attrs
            .get("recording_group")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|rg| rg.get("all_supported"))
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let include_global = attrs
            .get("recording_group")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|rg| rg.get("include_global_resource_types"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let recording_group = Some(RecordingGroupView {
            all_supported,
            include_global_resource_types: include_global,
        });

        let recording_mode = attrs.get("recording_mode").cloned();

        let recorder_view = ConfigurationRecorderView {
            name: name.clone(),
            role_arn,
            recording_group,
            recording: false,
            last_start_time: None,
            last_stop_time: None,
            recording_mode,
        };

        let mut state_view = minimal_config_state_view();
        state_view.recorders.insert(name, recorder_view);
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
        for recorder in view.recorders.values() {
            let attrs = serde_json::json!({
                "id": recorder.name,
                "name": recorder.name,
                "role_arn": recorder.role_arn,
                "recording_mode": recorder.recording_mode,
            });
            results.push(ExtractedResource {
                name: recorder.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_config_config_rule
// ---------------------------------------------------------------------------

pub struct AwsConfigConfigRuleConverter {
    service: Arc<ConfigService>,
}

impl AwsConfigConfigRuleConverter {
    pub fn new(service: Arc<ConfigService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsConfigConfigRuleConverter {
    fn resource_type(&self) -> &str {
        "aws_config_config_rule"
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

impl AwsConfigConfigRuleConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = require_str(attrs, "name", "aws_config_config_rule")?;
        let rule_id = optional_str(attrs, "rule_id")
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string().replace('-', "")[..8].to_string());
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:config:{}:{}:config-rule/config-rule-{}",
                region, ctx.default_account_id, rule_id
            )
        });
        let description = optional_str(attrs, "description");
        let input_parameters = optional_str(attrs, "input_parameters");

        let source_owner = attrs
            .get("source")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|s| s.get("owner"))
            .and_then(|v| v.as_str())
            .unwrap_or("AWS")
            .to_string();
        let source_identifier = attrs
            .get("source")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|s| s.get("source_identifier"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let scope_resource_types: Option<Vec<String>> = attrs
            .get("scope")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|s| s.get("compliance_resource_types"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            });

        let evaluation_mode = attrs.get("evaluation_mode").cloned();
        let scope_val = attrs.get("scope").cloned();

        let rule_view = ConfigRuleView {
            config_rule_name: name.to_string(),
            config_rule_arn: arn,
            config_rule_id: rule_id,
            config_rule_state: "ACTIVE".to_string(),
            description,
            source_owner,
            source_identifier,
            input_parameters,
            maximum_execution_frequency: None,
            scope_resource_types,
            evaluation_mode,
            scope: scope_val,
        };

        let mut state_view = minimal_config_state_view();
        state_view.config_rules.insert(name.to_string(), rule_view);
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
        for rule in view.config_rules.values() {
            let attrs = serde_json::json!({
                "id": rule.config_rule_name,
                "name": rule.config_rule_name,
                "arn": rule.config_rule_arn,
                "rule_id": rule.config_rule_id,
                "description": rule.description,
                "tags": {},
                "tags_all": {},
                "source": [{
                    "owner": rule.source_owner,
                    "source_identifier": rule.source_identifier,
                }],
                "evaluation_mode": rule.evaluation_mode,
                "scope": rule.scope,
            });
            results.push(ExtractedResource {
                name: rule.config_rule_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_config_delivery_channel
// ---------------------------------------------------------------------------

pub struct AwsConfigDeliveryChannelConverter {
    service: Arc<ConfigService>,
}

impl AwsConfigDeliveryChannelConverter {
    pub fn new(service: Arc<ConfigService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsConfigDeliveryChannelConverter {
    fn resource_type(&self) -> &str {
        "aws_config_delivery_channel"
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

impl AwsConfigDeliveryChannelConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = optional_str(attrs, "name").unwrap_or_else(|| "default".to_string());
        let s3_bucket_name = optional_str(attrs, "s3_bucket_name").unwrap_or_default();
        let s3_key_prefix = optional_str(attrs, "s3_key_prefix").unwrap_or_default();
        let snapshot_delivery_properties = attrs.get("snapshot_delivery_properties").cloned();

        let channel_view = DeliveryChannelView {
            name: name.clone(),
            s3_bucket_name,
            s3_key_prefix,
            snapshot_delivery_properties,
        };

        let mut state_view = minimal_config_state_view();
        state_view.delivery_channels.insert(name, channel_view);
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
        for ch in view.delivery_channels.values() {
            let attrs = serde_json::json!({
                "id": ch.name,
                "name": ch.name,
                "s3_bucket_name": ch.s3_bucket_name,
                "s3_key_prefix": ch.s3_key_prefix,
                "snapshot_delivery_properties": ch.snapshot_delivery_properties,
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
// Helpers
// ---------------------------------------------------------------------------

fn minimal_config_state_view() -> ConfigStateView {
    ConfigStateView::default()
}

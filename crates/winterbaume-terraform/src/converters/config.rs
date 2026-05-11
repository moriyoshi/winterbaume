//! Terraform converters for AWS Config resources.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_config::ConfigService;
use winterbaume_config::views::{
    AggregationAuthorizationView, ConfigRuleView, ConfigStateView, ConfigurationAggregatorView,
    ConfigurationRecorderView, DeliveryChannelView, OrganizationConfigRuleView,
    OrganizationConformancePackView, RecordingGroupView, RemediationConfigView,
    RetentionConfigurationView,
};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::config as config_gen;
use crate::util::{classify_deserialize_error, extract_region};

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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: config_gen::ConfigurationRecorderTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_config_configuration_recorder", e))?;

        let attrs = &instance.attributes;
        let name = model.name.unwrap_or_else(|| "default".to_string());
        let role_arn = model.role_arn.unwrap_or_default();

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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: config_gen::ConfigRuleTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_config_config_rule", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let rule_id = model
            .rule_id
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string().replace('-', "")[..8].to_string());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:config:{}:{}:config-rule/config-rule-{}",
                region, ctx.default_account_id, rule_id
            )
        });
        let description = model.description;
        let input_parameters = model.input_parameters;

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
            config_rule_name: name.clone(),
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
        state_view.config_rules.insert(name, rule_view);
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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: config_gen::DeliveryChannelTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_config_delivery_channel", e))?;

        let attrs = &instance.attributes;
        let name = model.name.unwrap_or_else(|| "default".to_string());
        let s3_bucket_name = model.s3_bucket_name.unwrap_or_default();
        let s3_key_prefix = model.s3_key_prefix.unwrap_or_default();
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
// aws_config_aggregate_authorization
// ---------------------------------------------------------------------------

pub struct AwsConfigAggregateAuthorizationConverter {
    service: Arc<ConfigService>,
}

impl AwsConfigAggregateAuthorizationConverter {
    pub fn new(service: Arc<ConfigService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsConfigAggregateAuthorizationConverter {
    fn resource_type(&self) -> &str {
        "aws_config_aggregate_authorization"
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

impl AwsConfigAggregateAuthorizationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: config_gen::AggregateAuthorizationTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_config_aggregate_authorization", e))?;

        let authorized_account_id = model.account_id.clone();
        let authorized_aws_region = model.region.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:config:{}:{}:aggregation-authorization/{}/{}",
                region, ctx.default_account_id, authorized_account_id, authorized_aws_region
            )
        });

        let key = format!("{}/{}", authorized_account_id, authorized_aws_region);
        let view = AggregationAuthorizationView {
            authorized_account_id,
            authorized_aws_region,
            aggregation_authorization_arn: arn,
            creation_time: 0.0,
        };

        let mut state_view = minimal_config_state_view();
        state_view.aggregation_authorizations.insert(key, view);
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
        for aa in view.aggregation_authorizations.values() {
            let attrs = serde_json::json!({
                "id": aa.aggregation_authorization_arn,
                "account_id": aa.authorized_account_id,
                "region": aa.authorized_aws_region,
                "arn": aa.aggregation_authorization_arn,
            });
            let name = format!("{}/{}", aa.authorized_account_id, aa.authorized_aws_region);
            results.push(ExtractedResource {
                name,
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_config_configuration_aggregator
// ---------------------------------------------------------------------------

pub struct AwsConfigConfigurationAggregatorConverter {
    service: Arc<ConfigService>,
}

impl AwsConfigConfigurationAggregatorConverter {
    pub fn new(service: Arc<ConfigService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsConfigConfigurationAggregatorConverter {
    fn resource_type(&self) -> &str {
        "aws_config_configuration_aggregator"
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

impl AwsConfigConfigurationAggregatorConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: config_gen::ConfigurationAggregatorTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_config_configuration_aggregator", e)
            })?;

        let name = model.name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:config:{}:{}:config-aggregator/config-aggregator-{}",
                region, ctx.default_account_id, name
            )
        });

        let view = ConfigurationAggregatorView {
            configuration_aggregator_name: name.clone(),
            configuration_aggregator_arn: arn,
            account_aggregation_sources: None,
            organization_aggregation_source: None,
            creation_time: 0.0,
            last_updated_time: 0.0,
        };

        let mut state_view = minimal_config_state_view();
        state_view.configuration_aggregators.insert(name, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![
                "aws_config_configuration_aggregator: nested account_aggregation_source/organization_aggregation_source blocks are not modelled"
                    .to_string(),
            ],
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
        for ca in view.configuration_aggregators.values() {
            let attrs = serde_json::json!({
                "id": ca.configuration_aggregator_name,
                "name": ca.configuration_aggregator_name,
                "arn": ca.configuration_aggregator_arn,
            });
            results.push(ExtractedResource {
                name: ca.configuration_aggregator_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_config_configuration_recorder_status
// ---------------------------------------------------------------------------

pub struct AwsConfigConfigurationRecorderStatusConverter {
    service: Arc<ConfigService>,
}

impl AwsConfigConfigurationRecorderStatusConverter {
    pub fn new(service: Arc<ConfigService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsConfigConfigurationRecorderStatusConverter {
    fn resource_type(&self) -> &str {
        "aws_config_configuration_recorder_status"
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

impl AwsConfigConfigurationRecorderStatusConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: config_gen::ConfigurationRecorderStatusTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_config_configuration_recorder_status", e)
            })?;

        let name = model.name.clone();
        let is_enabled = model.is_enabled;

        // Merge an updated recorder view if one already exists; otherwise create a
        // minimal placeholder so that downstream describe-status APIs can see it.
        let existing = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut state_view = minimal_config_state_view();
        let recorder_view = match existing.recorders.get(&name) {
            Some(r) => ConfigurationRecorderView {
                recording: is_enabled,
                ..r.clone()
            },
            None => ConfigurationRecorderView {
                name: name.clone(),
                role_arn: String::new(),
                recording_group: Some(RecordingGroupView {
                    all_supported: true,
                    include_global_resource_types: false,
                }),
                recording: is_enabled,
                last_start_time: None,
                last_stop_time: None,
                recording_mode: None,
            },
        };
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
                "is_enabled": recorder.recording,
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
// aws_config_conformance_pack
// ---------------------------------------------------------------------------
//
// The Config service crate does not (yet) model standalone account-level
// conformance packs separately from organization conformance packs. Injection
// is a no-op that emits a warning so the conversion does not silently
// disappear, and extraction always returns an empty list.

pub struct AwsConfigConformancePackConverter {
    #[allow(dead_code)]
    service: Arc<ConfigService>,
}

impl AwsConfigConformancePackConverter {
    pub fn new(service: Arc<ConfigService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsConfigConformancePackConverter {
    fn resource_type(&self) -> &str {
        "aws_config_conformance_pack"
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

impl AwsConfigConformancePackConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: config_gen::ConformancePackTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_config_conformance_pack", e))?;

        Ok(ConversionResult {
            region,
            warnings: vec![
                "aws_config_conformance_pack: standalone conformance packs are not modelled in the winterbaume-config state; resource ignored"
                    .to_string(),
            ],
        })
    }

    async fn do_extract(
        &self,
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// aws_config_organization_conformance_pack
// ---------------------------------------------------------------------------

pub struct AwsConfigOrganizationConformancePackConverter {
    service: Arc<ConfigService>,
}

impl AwsConfigOrganizationConformancePackConverter {
    pub fn new(service: Arc<ConfigService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsConfigOrganizationConformancePackConverter {
    fn resource_type(&self) -> &str {
        "aws_config_organization_conformance_pack"
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

impl AwsConfigOrganizationConformancePackConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: config_gen::OrganizationConformancePackTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_config_organization_conformance_pack", e)
            })?;

        let name = model.name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:config:{}:{}:organization-conformance-pack/{}",
                region, ctx.default_account_id, name
            )
        });

        let view = OrganizationConformancePackView {
            organization_conformance_pack_name: name.clone(),
            organization_conformance_pack_arn: arn,
            delivery_s3_bucket: model.delivery_s3_bucket,
            delivery_s3_key_prefix: model.delivery_s3_key_prefix,
            excluded_accounts: None,
            conformance_pack_input_parameters: None,
            last_update_time: 0.0,
        };

        let mut state_view = minimal_config_state_view();
        state_view.organization_conformance_packs.insert(name, view);
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
        for ocp in view.organization_conformance_packs.values() {
            let attrs = serde_json::json!({
                "id": ocp.organization_conformance_pack_name,
                "name": ocp.organization_conformance_pack_name,
                "arn": ocp.organization_conformance_pack_arn,
                "delivery_s3_bucket": ocp.delivery_s3_bucket,
                "delivery_s3_key_prefix": ocp.delivery_s3_key_prefix,
            });
            results.push(ExtractedResource {
                name: ocp.organization_conformance_pack_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_config_organization_custom_policy_rule
// ---------------------------------------------------------------------------

pub struct AwsConfigOrganizationCustomPolicyRuleConverter {
    service: Arc<ConfigService>,
}

impl AwsConfigOrganizationCustomPolicyRuleConverter {
    pub fn new(service: Arc<ConfigService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsConfigOrganizationCustomPolicyRuleConverter {
    fn resource_type(&self) -> &str {
        "aws_config_organization_custom_policy_rule"
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

impl AwsConfigOrganizationCustomPolicyRuleConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: config_gen::OrganizationCustomPolicyRuleTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_config_organization_custom_policy_rule", e)
            })?;

        let name = model.name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:config:{}:{}:organization-config-rule/{}",
                region, ctx.default_account_id, name
            )
        });

        let custom_policy_rule_metadata = Some(serde_json::json!({
            "description": model.description,
            "policy_runtime": model.policy_runtime,
            "policy_text": model.policy_text,
            "input_parameters": model.input_parameters,
            "maximum_execution_frequency": model.maximum_execution_frequency,
        }));

        let view = OrganizationConfigRuleView {
            organization_config_rule_name: name.clone(),
            organization_config_rule_arn: arn,
            excluded_accounts: None,
            last_update_time: 0.0,
            managed_rule_metadata: None,
            custom_rule_metadata: None,
            custom_policy_rule_metadata,
        };

        let mut state_view = minimal_config_state_view();
        state_view.organization_config_rules.insert(name, view);
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
        for r in view.organization_config_rules.values() {
            let Some(meta) = r.custom_policy_rule_metadata.as_ref() else {
                continue;
            };
            let attrs = serde_json::json!({
                "id": r.organization_config_rule_name,
                "name": r.organization_config_rule_name,
                "arn": r.organization_config_rule_arn,
                "description": meta.get("description"),
                "policy_runtime": meta.get("policy_runtime"),
                "policy_text": meta.get("policy_text"),
                "input_parameters": meta.get("input_parameters"),
                "maximum_execution_frequency": meta.get("maximum_execution_frequency"),
            });
            results.push(ExtractedResource {
                name: r.organization_config_rule_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_config_organization_custom_rule
// ---------------------------------------------------------------------------

pub struct AwsConfigOrganizationCustomRuleConverter {
    service: Arc<ConfigService>,
}

impl AwsConfigOrganizationCustomRuleConverter {
    pub fn new(service: Arc<ConfigService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsConfigOrganizationCustomRuleConverter {
    fn resource_type(&self) -> &str {
        "aws_config_organization_custom_rule"
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

impl AwsConfigOrganizationCustomRuleConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: config_gen::OrganizationCustomRuleTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_config_organization_custom_rule", e)
            })?;

        let name = model.name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:config:{}:{}:organization-config-rule/{}",
                region, ctx.default_account_id, name
            )
        });

        let custom_rule_metadata = Some(serde_json::json!({
            "description": model.description,
            "lambda_function_arn": model.lambda_function_arn,
            "input_parameters": model.input_parameters,
            "maximum_execution_frequency": model.maximum_execution_frequency,
        }));

        let view = OrganizationConfigRuleView {
            organization_config_rule_name: name.clone(),
            organization_config_rule_arn: arn,
            excluded_accounts: None,
            last_update_time: 0.0,
            managed_rule_metadata: None,
            custom_rule_metadata,
            custom_policy_rule_metadata: None,
        };

        let mut state_view = minimal_config_state_view();
        state_view.organization_config_rules.insert(name, view);
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
        for r in view.organization_config_rules.values() {
            let Some(meta) = r.custom_rule_metadata.as_ref() else {
                continue;
            };
            let attrs = serde_json::json!({
                "id": r.organization_config_rule_name,
                "name": r.organization_config_rule_name,
                "arn": r.organization_config_rule_arn,
                "description": meta.get("description"),
                "lambda_function_arn": meta.get("lambda_function_arn"),
                "input_parameters": meta.get("input_parameters"),
                "maximum_execution_frequency": meta.get("maximum_execution_frequency"),
            });
            results.push(ExtractedResource {
                name: r.organization_config_rule_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_config_organization_managed_rule
// ---------------------------------------------------------------------------

pub struct AwsConfigOrganizationManagedRuleConverter {
    service: Arc<ConfigService>,
}

impl AwsConfigOrganizationManagedRuleConverter {
    pub fn new(service: Arc<ConfigService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsConfigOrganizationManagedRuleConverter {
    fn resource_type(&self) -> &str {
        "aws_config_organization_managed_rule"
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

impl AwsConfigOrganizationManagedRuleConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: config_gen::OrganizationManagedRuleTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_config_organization_managed_rule", e)
            })?;

        let name = model.name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:config:{}:{}:organization-config-rule/{}",
                region, ctx.default_account_id, name
            )
        });

        let managed_rule_metadata = Some(serde_json::json!({
            "description": model.description,
            "rule_identifier": model.rule_identifier,
            "input_parameters": model.input_parameters,
            "maximum_execution_frequency": model.maximum_execution_frequency,
        }));

        let view = OrganizationConfigRuleView {
            organization_config_rule_name: name.clone(),
            organization_config_rule_arn: arn,
            excluded_accounts: None,
            last_update_time: 0.0,
            managed_rule_metadata,
            custom_rule_metadata: None,
            custom_policy_rule_metadata: None,
        };

        let mut state_view = minimal_config_state_view();
        state_view.organization_config_rules.insert(name, view);
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
        for r in view.organization_config_rules.values() {
            let Some(meta) = r.managed_rule_metadata.as_ref() else {
                continue;
            };
            let attrs = serde_json::json!({
                "id": r.organization_config_rule_name,
                "name": r.organization_config_rule_name,
                "arn": r.organization_config_rule_arn,
                "description": meta.get("description"),
                "rule_identifier": meta.get("rule_identifier"),
                "input_parameters": meta.get("input_parameters"),
                "maximum_execution_frequency": meta.get("maximum_execution_frequency"),
            });
            results.push(ExtractedResource {
                name: r.organization_config_rule_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_config_remediation_configuration
// ---------------------------------------------------------------------------

pub struct AwsConfigRemediationConfigurationConverter {
    service: Arc<ConfigService>,
}

impl AwsConfigRemediationConfigurationConverter {
    pub fn new(service: Arc<ConfigService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsConfigRemediationConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_config_remediation_configuration"
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

impl AwsConfigRemediationConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: config_gen::RemediationConfigurationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_config_remediation_configuration", e)
            })?;

        let config_rule_name = model.config_rule_name.clone();
        let view = RemediationConfigView {
            config_rule_name: config_rule_name.clone(),
            target_type: model
                .target_type
                .unwrap_or_else(|| "SSM_DOCUMENT".to_string()),
            target_id: model.target_id.unwrap_or_default(),
            target_version: model.target_version,
            automatic: Some(model.automatic),
            maximum_automatic_attempts: if model.maximum_automatic_attempts == 0 {
                None
            } else {
                Some(model.maximum_automatic_attempts as i32)
            },
            retry_attempt_seconds: if model.retry_attempt_seconds == 0 {
                None
            } else {
                Some(model.retry_attempt_seconds)
            },
            resource_type: model.resource_type,
            parameters: None,
        };

        let mut state_view = minimal_config_state_view();
        state_view
            .remediation_configs
            .insert(config_rule_name, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![
                "aws_config_remediation_configuration: parameter block contents are not modelled"
                    .to_string(),
            ],
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
        for rc in view.remediation_configs.values() {
            let arn = format!(
                "arn:aws:config:{}:{}:remediation-configuration/{}",
                ctx.default_region, ctx.default_account_id, rc.config_rule_name
            );
            let attrs = serde_json::json!({
                "id": rc.config_rule_name,
                "config_rule_name": rc.config_rule_name,
                "arn": arn,
                "target_type": rc.target_type,
                "target_id": rc.target_id,
                "target_version": rc.target_version,
                "resource_type": rc.resource_type,
                "automatic": rc.automatic.unwrap_or(false),
                "maximum_automatic_attempts": rc.maximum_automatic_attempts.unwrap_or(0),
                "retry_attempt_seconds": rc.retry_attempt_seconds.unwrap_or(0),
            });
            results.push(ExtractedResource {
                name: rc.config_rule_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_config_retention_configuration
// ---------------------------------------------------------------------------

pub struct AwsConfigRetentionConfigurationConverter {
    service: Arc<ConfigService>,
}

impl AwsConfigRetentionConfigurationConverter {
    pub fn new(service: Arc<ConfigService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsConfigRetentionConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_config_retention_configuration"
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

impl AwsConfigRetentionConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: config_gen::RetentionConfigurationTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_config_retention_configuration", e))?;

        let name = model.name.unwrap_or_else(|| "default".to_string());
        let view = RetentionConfigurationView {
            name: name.clone(),
            retention_period_in_days: model.retention_period_in_days as i32,
        };

        let mut state_view = minimal_config_state_view();
        state_view.retention_configurations.insert(name, view);
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
        for rc in view.retention_configurations.values() {
            let attrs = serde_json::json!({
                "id": rc.name,
                "name": rc.name,
                "retention_period_in_days": rc.retention_period_in_days,
            });
            results.push(ExtractedResource {
                name: rc.name.clone(),
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

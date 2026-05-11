//! Terraform converters for SES resources.
//!
//! `EmailIdentityTfModel`, `ConfigurationSetTfModel`, and
//! `DedicatedIpPoolTfModel` are generated from `specs/ses.toml`. The
//! identity_type derivation (DOMAIN vs EMAIL_ADDRESS), the verification
//! and feedback-forwarding constants, the dkim_signing_attributes
//! parsing, the configuration-set raw-blob nested options
//! (delivery_options / reputation_options / sending_options /
//! suppression_options / tracking_options / vdm_options), and the
//! `scaling_mode = "STANDARD"` default are wired up here. The SES
//! tags merge (tags + tags_all) is handled by extract_tags() so that
//! email_identity preserves the original tags_all behaviour.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_core::StatefulService;
use winterbaume_sesv2::SesV2Service;
use winterbaume_sesv2::views::{
    ConfigurationSetView, ContactListView, DedicatedIpPoolView, DedicatedIpView, EmailIdentityView,
    EventDestinationView, SesStateView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::ses as ses_gen;
use crate::util::{classify_deserialize_error, extract_region, extract_tags};

// ---------------------------------------------------------------------------
// aws_sesv2_email_identity
// ---------------------------------------------------------------------------

pub struct AwsSesv2EmailIdentityConverter {
    service: Arc<SesV2Service>,
}

impl AwsSesv2EmailIdentityConverter {
    pub fn new(service: Arc<SesV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSesv2EmailIdentityConverter {
    fn resource_type(&self) -> &str {
        "aws_sesv2_email_identity"
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

impl AwsSesv2EmailIdentityConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: ses_gen::EmailIdentityTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_sesv2_email_identity", e))?;

        let email_identity = model.email_identity;

        // Determine identity type: domain vs email address
        let identity_type = if email_identity.contains('@') {
            "EMAIL_ADDRESS"
        } else {
            "DOMAIN"
        }
        .to_string();

        let tags = extract_tags(attrs);

        let configuration_set_name = model.configuration_set_name;

        let dkim_signing_enabled = attrs
            .get("dkim_signing_attributes")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|obj| obj.get("signing_enabled"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let dkim_signing_key_type = attrs
            .get("dkim_signing_attributes")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|obj| obj.get("signing_key_type"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let identity_view = EmailIdentityView {
            name: email_identity.clone(),
            identity_type,
            verified: true,
            created_timestamp: None,
            policies: HashMap::new(),
            tags,
            configuration_set_name,
            dkim_signing_enabled,
            dkim_signing_key_type,
            dkim_domain: None,
            feedback_forwarding_enabled: true,
            mail_from_domain: None,
            behavior_on_mx_failure: None,
        };

        let mut state_view = minimal_ses_state_view();
        state_view.identities.insert(email_identity, identity_view);
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
        for identity in view.identities.values() {
            let attrs = serde_json::json!({
                "id": identity.name,
                "email_identity": identity.name,
                "identity_type": identity.identity_type,
                "verified": identity.verified,
                "configuration_set_name": identity.configuration_set_name,
                "dkim_signing_attributes": [{
                    "signing_enabled": identity.dkim_signing_enabled,
                    "signing_key_type": identity.dkim_signing_key_type,
                    "domain_signing_selector": identity.dkim_domain,
                }],
                "tags": identity.tags,
            });
            results.push(ExtractedResource {
                name: identity.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_sesv2_configuration_set
// ---------------------------------------------------------------------------

pub struct AwsSesv2ConfigurationSetConverter {
    service: Arc<SesV2Service>,
}

impl AwsSesv2ConfigurationSetConverter {
    pub fn new(service: Arc<SesV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSesv2ConfigurationSetConverter {
    fn resource_type(&self) -> &str {
        "aws_sesv2_configuration_set"
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

impl AwsSesv2ConfigurationSetConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: ses_gen::ConfigurationSetTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_sesv2_configuration_set", e))?;

        let name = model.configuration_set_name;

        // delivery_options block
        let delivery_options = attrs
            .get("delivery_options")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .cloned();

        // reputation_options block
        let reputation_options = attrs
            .get("reputation_options")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .cloned();

        // sending_options block
        let sending_options = attrs
            .get("sending_options")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .cloned();

        // suppression_options block
        let suppression_options = attrs
            .get("suppression_options")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .cloned();

        // tracking_options block
        let tracking_options = attrs
            .get("tracking_options")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .cloned();

        // vdm_options block
        let vdm_options = attrs
            .get("vdm_options")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .cloned();

        let cs_view = ConfigurationSetView {
            name: name.clone(),
            tags: model.tags,
            event_destinations: HashMap::new(),
            archiving_options: None,
            delivery_options,
            reputation_options,
            sending_options,
            suppression_options,
            tracking_options,
            vdm_options,
        };

        let mut state_view = minimal_ses_state_view();
        state_view.configuration_sets.insert(name, cs_view);
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
        for cs in view.configuration_sets.values() {
            let attrs = serde_json::json!({
                "id": cs.name,
                "configuration_set_name": cs.name,
                "delivery_options": cs.delivery_options.as_ref().map(|v| vec![v.clone()]).unwrap_or_default(),
                "reputation_options": cs.reputation_options.as_ref().map(|v| vec![v.clone()]).unwrap_or_default(),
                "sending_options": cs.sending_options.as_ref().map(|v| vec![v.clone()]).unwrap_or_default(),
                "suppression_options": cs.suppression_options.as_ref().map(|v| vec![v.clone()]).unwrap_or_default(),
                "tracking_options": cs.tracking_options.as_ref().map(|v| vec![v.clone()]).unwrap_or_default(),
                "vdm_options": cs.vdm_options.as_ref().map(|v| vec![v.clone()]).unwrap_or_default(),
                "tags": cs.tags,
            });
            results.push(ExtractedResource {
                name: cs.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_sesv2_dedicated_ip_pool
// ---------------------------------------------------------------------------

pub struct AwsSesv2DedicatedIpPoolConverter {
    service: Arc<SesV2Service>,
}

impl AwsSesv2DedicatedIpPoolConverter {
    pub fn new(service: Arc<SesV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSesv2DedicatedIpPoolConverter {
    fn resource_type(&self) -> &str {
        "aws_sesv2_dedicated_ip_pool"
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

impl AwsSesv2DedicatedIpPoolConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: ses_gen::DedicatedIpPoolTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_sesv2_dedicated_ip_pool", e))?;

        let pool_name = model.pool_name;
        let scaling_mode = model.scaling_mode.unwrap_or_else(|| "STANDARD".to_string());

        let pool_view = DedicatedIpPoolView {
            pool_name: pool_name.clone(),
            scaling_mode,
            tags: model.tags,
        };

        let mut state_view = minimal_ses_state_view();
        state_view.dedicated_ip_pools.insert(pool_name, pool_view);
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
        for pool in view.dedicated_ip_pools.values() {
            let attrs = serde_json::json!({
                "id": pool.pool_name,
                "pool_name": pool.pool_name,
                "scaling_mode": pool.scaling_mode,
                "tags": pool.tags,
            });
            results.push(ExtractedResource {
                name: pool.pool_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_sesv2_account_suppression_attributes
// ---------------------------------------------------------------------------

pub struct AwsSesv2AccountSuppressionAttributesConverter {
    service: Arc<SesV2Service>,
}

impl AwsSesv2AccountSuppressionAttributesConverter {
    pub fn new(service: Arc<SesV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSesv2AccountSuppressionAttributesConverter {
    fn resource_type(&self) -> &str {
        "aws_sesv2_account_suppression_attributes"
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

impl AwsSesv2AccountSuppressionAttributesConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let _model: ses_gen::AccountSuppressionAttributesTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_sesv2_account_suppression_attributes", e)
            })?;

        // suppressed_reasons is a list<string> in TF; read it directly.
        let suppressed_reasons: Vec<String> = attrs
            .get("suppressed_reasons")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        state_view.account_settings.suppressed_reasons = suppressed_reasons;
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
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
        let attrs = serde_json::json!({
            "id": ctx.default_account_id,
            "suppressed_reasons": view.account_settings.suppressed_reasons,
        });
        Ok(vec![ExtractedResource {
            name: ctx.default_account_id.clone(),
            account_id: ctx.default_account_id.clone(),
            region: ctx.default_region.clone(),
            attributes: attrs,
        }])
    }
}

// ---------------------------------------------------------------------------
// aws_sesv2_account_vdm_attributes
// ---------------------------------------------------------------------------

pub struct AwsSesv2AccountVdmAttributesConverter {
    service: Arc<SesV2Service>,
}

impl AwsSesv2AccountVdmAttributesConverter {
    pub fn new(service: Arc<SesV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSesv2AccountVdmAttributesConverter {
    fn resource_type(&self) -> &str {
        "aws_sesv2_account_vdm_attributes"
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

impl AwsSesv2AccountVdmAttributesConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: ses_gen::AccountVdmAttributesTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_sesv2_account_vdm_attributes", e))?;

        // dashboard_attributes / guardian_attributes are nested blocks; read
        // them as raw JSON values.
        let dashboard_attributes = attrs
            .get("dashboard_attributes")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .cloned();
        let guardian_attributes = attrs
            .get("guardian_attributes")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .cloned();

        let mut vdm = serde_json::Map::new();
        if let Some(enabled) = model.vdm_enabled {
            vdm.insert(
                "vdm_enabled".to_string(),
                serde_json::Value::String(enabled),
            );
        }
        if let Some(da) = dashboard_attributes {
            vdm.insert("dashboard_attributes".to_string(), da);
        }
        if let Some(ga) = guardian_attributes {
            vdm.insert("guardian_attributes".to_string(), ga);
        }

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        state_view.account_settings.vdm_attributes = Some(serde_json::Value::Object(vdm));
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
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
        let Some(vdm) = view.account_settings.vdm_attributes.as_ref() else {
            return Ok(vec![]);
        };
        let vdm_enabled = vdm
            .get("vdm_enabled")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let dashboard_attributes = vdm
            .get("dashboard_attributes")
            .cloned()
            .map(|v| vec![v])
            .unwrap_or_default();
        let guardian_attributes = vdm
            .get("guardian_attributes")
            .cloned()
            .map(|v| vec![v])
            .unwrap_or_default();
        let attrs = serde_json::json!({
            "id": ctx.default_account_id,
            "vdm_enabled": vdm_enabled,
            "dashboard_attributes": dashboard_attributes,
            "guardian_attributes": guardian_attributes,
        });
        Ok(vec![ExtractedResource {
            name: ctx.default_account_id.clone(),
            account_id: ctx.default_account_id.clone(),
            region: ctx.default_region.clone(),
            attributes: attrs,
        }])
    }
}

// ---------------------------------------------------------------------------
// aws_sesv2_configuration_set_event_destination
// ---------------------------------------------------------------------------

pub struct AwsSesv2ConfigurationSetEventDestinationConverter {
    service: Arc<SesV2Service>,
}

impl AwsSesv2ConfigurationSetEventDestinationConverter {
    pub fn new(service: Arc<SesV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSesv2ConfigurationSetEventDestinationConverter {
    fn resource_type(&self) -> &str {
        "aws_sesv2_configuration_set_event_destination"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_sesv2_configuration_set"]
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

impl AwsSesv2ConfigurationSetEventDestinationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: ses_gen::ConfigurationSetEventDestinationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_sesv2_configuration_set_event_destination", e)
            })?;

        // event_destination is a single-block list with matching_event_types
        // + a destination sub-block (cloudwatch_destination /
        // kinesis_firehose_destination / pinpoint_destination /
        // sns_destination / event_bridge_destination).
        let ed_block = attrs
            .get("event_destination")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .cloned()
            .unwrap_or(serde_json::Value::Null);

        let enabled = ed_block
            .get("enabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let matching_event_types: Vec<String> = ed_block
            .get("matching_event_types")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let destination = ed_block.clone();

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(cs) = state_view
            .configuration_sets
            .get_mut(&model.configuration_set_name)
        {
            cs.event_destinations.insert(
                model.event_destination_name.clone(),
                EventDestinationView {
                    name: model.event_destination_name.clone(),
                    enabled,
                    matching_event_types,
                    destination,
                },
            );
        } else {
            warnings.push(format!(
                "aws_sesv2_configuration_set_event_destination: configuration set '{}' not found in state; event destination '{}' skipped",
                model.configuration_set_name, model.event_destination_name
            ));
        }
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult { region, warnings })
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
        for cs in view.configuration_sets.values() {
            for dest in cs.event_destinations.values() {
                let id = format!("{}|{}", cs.name, dest.name);
                let attrs = serde_json::json!({
                    "id": id,
                    "configuration_set_name": cs.name,
                    "event_destination_name": dest.name,
                    "event_destination": [{
                        "enabled": dest.enabled,
                        "matching_event_types": dest.matching_event_types,
                    }],
                });
                results.push(ExtractedResource {
                    name: id,
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_sesv2_contact_list
// ---------------------------------------------------------------------------

pub struct AwsSesv2ContactListConverter {
    service: Arc<SesV2Service>,
}

impl AwsSesv2ContactListConverter {
    pub fn new(service: Arc<SesV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSesv2ContactListConverter {
    fn resource_type(&self) -> &str {
        "aws_sesv2_contact_list"
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

impl AwsSesv2ContactListConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: ses_gen::ContactListTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_sesv2_contact_list", e))?;

        let tags = extract_tags(attrs);
        let now = Utc::now().to_rfc3339();

        let list_view = ContactListView {
            name: model.contact_list_name.clone(),
            description: model.description,
            tags,
            created_timestamp: Some(now.clone()),
            last_updated_timestamp: Some(now),
            contacts: HashMap::new(),
        };

        let mut state_view = minimal_ses_state_view();
        state_view
            .contact_lists
            .insert(model.contact_list_name, list_view);
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
        for cl in view.contact_lists.values() {
            let attrs = serde_json::json!({
                "id": cl.name,
                "contact_list_name": cl.name,
                "description": cl.description,
                "tags": cl.tags,
                "created_timestamp": cl.created_timestamp,
                "last_updated_timestamp": cl.last_updated_timestamp,
            });
            results.push(ExtractedResource {
                name: cl.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_sesv2_dedicated_ip_assignment
// ---------------------------------------------------------------------------

pub struct AwsSesv2DedicatedIpAssignmentConverter {
    service: Arc<SesV2Service>,
}

impl AwsSesv2DedicatedIpAssignmentConverter {
    pub fn new(service: Arc<SesV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSesv2DedicatedIpAssignmentConverter {
    fn resource_type(&self) -> &str {
        "aws_sesv2_dedicated_ip_assignment"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_sesv2_dedicated_ip_pool"]
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

impl AwsSesv2DedicatedIpAssignmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: ses_gen::DedicatedIpAssignmentTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_sesv2_dedicated_ip_assignment", e))?;

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(ip) = state_view.dedicated_ips.get_mut(&model.ip) {
            ip.pool_name = Some(model.destination_pool_name.clone());
        } else {
            warnings.push(format!(
                "aws_sesv2_dedicated_ip_assignment: dedicated IP '{}' not found in state; creating placeholder",
                model.ip
            ));
            state_view.dedicated_ips.insert(
                model.ip.clone(),
                DedicatedIpView {
                    ip: model.ip.clone(),
                    warmup_status: "IN_PROGRESS".to_string(),
                    warmup_percentage: 0,
                    pool_name: Some(model.destination_pool_name.clone()),
                },
            );
        }
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult { region, warnings })
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
        for ip in view.dedicated_ips.values() {
            let Some(pool) = ip.pool_name.as_ref() else {
                continue;
            };
            let id = format!("{}|{}", ip.ip, pool);
            let attrs = serde_json::json!({
                "id": id,
                "ip": ip.ip,
                "destination_pool_name": pool,
            });
            results.push(ExtractedResource {
                name: id,
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_sesv2_email_identity_feedback_attributes
// ---------------------------------------------------------------------------

pub struct AwsSesv2EmailIdentityFeedbackAttributesConverter {
    service: Arc<SesV2Service>,
}

impl AwsSesv2EmailIdentityFeedbackAttributesConverter {
    pub fn new(service: Arc<SesV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSesv2EmailIdentityFeedbackAttributesConverter {
    fn resource_type(&self) -> &str {
        "aws_sesv2_email_identity_feedback_attributes"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_sesv2_email_identity"]
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

impl AwsSesv2EmailIdentityFeedbackAttributesConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: ses_gen::EmailIdentityFeedbackAttributesTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_sesv2_email_identity_feedback_attributes", e)
            })?;

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(identity) = state_view.identities.get_mut(&model.email_identity) {
            identity.feedback_forwarding_enabled = model.email_forwarding_enabled;
        } else {
            warnings.push(format!(
                "aws_sesv2_email_identity_feedback_attributes: identity '{}' not found in state; feedback attributes skipped",
                model.email_identity
            ));
        }
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult { region, warnings })
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
        for identity in view.identities.values() {
            let attrs = serde_json::json!({
                "id": identity.name,
                "email_identity": identity.name,
                "email_forwarding_enabled": identity.feedback_forwarding_enabled,
            });
            results.push(ExtractedResource {
                name: identity.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_sesv2_email_identity_mail_from_attributes
// ---------------------------------------------------------------------------

pub struct AwsSesv2EmailIdentityMailFromAttributesConverter {
    service: Arc<SesV2Service>,
}

impl AwsSesv2EmailIdentityMailFromAttributesConverter {
    pub fn new(service: Arc<SesV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSesv2EmailIdentityMailFromAttributesConverter {
    fn resource_type(&self) -> &str {
        "aws_sesv2_email_identity_mail_from_attributes"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_sesv2_email_identity"]
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

impl AwsSesv2EmailIdentityMailFromAttributesConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: ses_gen::EmailIdentityMailFromAttributesTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_sesv2_email_identity_mail_from_attributes", e)
            })?;

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(identity) = state_view.identities.get_mut(&model.email_identity) {
            identity.mail_from_domain = model.mail_from_domain;
            identity.behavior_on_mx_failure = model.behavior_on_mx_failure;
        } else {
            warnings.push(format!(
                "aws_sesv2_email_identity_mail_from_attributes: identity '{}' not found in state; mail-from attributes skipped",
                model.email_identity
            ));
        }
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult { region, warnings })
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
        for identity in view.identities.values() {
            if identity.mail_from_domain.is_none() && identity.behavior_on_mx_failure.is_none() {
                continue;
            }
            let attrs = serde_json::json!({
                "id": identity.name,
                "email_identity": identity.name,
                "mail_from_domain": identity.mail_from_domain,
                "behavior_on_mx_failure": identity.behavior_on_mx_failure,
            });
            results.push(ExtractedResource {
                name: identity.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_sesv2_email_identity_policy
// ---------------------------------------------------------------------------

pub struct AwsSesv2EmailIdentityPolicyConverter {
    service: Arc<SesV2Service>,
}

impl AwsSesv2EmailIdentityPolicyConverter {
    pub fn new(service: Arc<SesV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSesv2EmailIdentityPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_sesv2_email_identity_policy"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_sesv2_email_identity"]
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

impl AwsSesv2EmailIdentityPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: ses_gen::EmailIdentityPolicyTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_sesv2_email_identity_policy", e))?;

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(identity) = state_view.identities.get_mut(&model.email_identity) {
            identity.policies.insert(model.policy_name, model.policy);
        } else {
            warnings.push(format!(
                "aws_sesv2_email_identity_policy: identity '{}' not found in state; policy '{}' skipped",
                model.email_identity, model.policy_name
            ));
        }
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult { region, warnings })
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
        for identity in view.identities.values() {
            for (policy_name, policy) in &identity.policies {
                let id = format!("{}|{}", identity.name, policy_name);
                let attrs = serde_json::json!({
                    "id": id,
                    "email_identity": identity.name,
                    "policy_name": policy_name,
                    "policy": policy,
                });
                results.push(ExtractedResource {
                    name: id,
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn minimal_ses_state_view() -> SesStateView {
    SesStateView::default()
}

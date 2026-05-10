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

use winterbaume_core::StatefulService;
use winterbaume_sesv2::SesV2Service;
use winterbaume_sesv2::views::{
    ConfigurationSetView, DedicatedIpPoolView, EmailIdentityView, SesStateView,
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
// Helpers
// ---------------------------------------------------------------------------

fn minimal_ses_state_view() -> SesStateView {
    SesStateView::default()
}

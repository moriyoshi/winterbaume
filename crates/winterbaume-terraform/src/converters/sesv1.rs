//! Terraform converters for SES v1 (2010-12-01) resources.
//!
//! The `*TfModel` structs are generated from `specs/sesv1.toml`. The ARN
//! templates and the shared `IdentityView` constant fields (verification
//! status, DKIM tokens, forwarding), as well as the snapshot-and-mutate
//! plumbing for modifier-style resources, are wired up here.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_core::StatefulService;
use winterbaume_ses::SesService;
use winterbaume_ses::views::{
    ConfigurationSetView, EventDestinationView, IdentityView, ReceiptRuleSetView, ReceiptRuleView,
    SesV1StateView, TemplateView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::sesv1 as sesv1_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_ses_email_identity
// ---------------------------------------------------------------------------

pub struct AwsSesEmailIdentityConverter {
    service: Arc<SesService>,
}

impl AwsSesEmailIdentityConverter {
    pub fn new(service: Arc<SesService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSesEmailIdentityConverter {
    fn resource_type(&self) -> &str {
        "aws_ses_email_identity"
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

impl AwsSesEmailIdentityConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: sesv1_gen::EmailIdentityTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ses_email_identity", e))?;

        let _arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:ses:{}:{}:identity/{}",
                region, ctx.default_account_id, model.email
            )
        });

        let identity_view = IdentityView {
            name: model.email.clone(),
            identity_type: "EmailAddress".to_string(),
            verification_status: "Success".to_string(),
            verification_token: None,
            dkim_tokens: vec![],
            dkim_enabled: false,
            mail_from_domain: None,
            bounce_topic: None,
            complaint_topic: None,
            delivery_topic: None,
            forwarding_enabled: true,
        };

        let mut state_view = SesV1StateView::default();
        state_view.identities.insert(model.email, identity_view);
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
            if identity.identity_type != "EmailAddress" {
                continue;
            }
            let attrs = serde_json::json!({
                "id": identity.name,
                "email": identity.name,
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
// aws_ses_domain_identity
// ---------------------------------------------------------------------------

pub struct AwsSesDomainIdentityConverter {
    service: Arc<SesService>,
}

impl AwsSesDomainIdentityConverter {
    pub fn new(service: Arc<SesService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSesDomainIdentityConverter {
    fn resource_type(&self) -> &str {
        "aws_ses_domain_identity"
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

impl AwsSesDomainIdentityConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: sesv1_gen::DomainIdentityTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ses_domain_identity", e))?;

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:ses:{}:{}:identity/{}",
                region, ctx.default_account_id, model.domain
            )
        });

        let identity_view = IdentityView {
            name: model.domain.clone(),
            identity_type: "Domain".to_string(),
            verification_status: "Success".to_string(),
            verification_token: Some(model.verification_token.unwrap_or_default()),
            dkim_tokens: vec![],
            dkim_enabled: false,
            mail_from_domain: None,
            bounce_topic: None,
            complaint_topic: None,
            delivery_topic: None,
            forwarding_enabled: true,
        };

        let mut state_view = SesV1StateView::default();
        state_view.identities.insert(model.domain, identity_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        drop(arn); // synthesized but not stored in view
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
            if identity.identity_type != "Domain" {
                continue;
            }
            let arn = format!(
                "arn:aws:ses:{}:{}:identity/{}",
                ctx.default_region, ctx.default_account_id, identity.name
            );
            let attrs = serde_json::json!({
                "id": identity.name,
                "domain": identity.name,
                "arn": arn,
                "verification_token": identity.verification_token,
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
// aws_ses_active_receipt_rule_set
// ---------------------------------------------------------------------------

pub struct AwsSesActiveReceiptRuleSetConverter {
    service: Arc<SesService>,
}

impl AwsSesActiveReceiptRuleSetConverter {
    pub fn new(service: Arc<SesService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSesActiveReceiptRuleSetConverter {
    fn resource_type(&self) -> &str {
        "aws_ses_active_receipt_rule_set"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ses_receipt_rule_set"]
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

impl AwsSesActiveReceiptRuleSetConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: sesv1_gen::ActiveReceiptRuleSetTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ses_active_receipt_rule_set", e))?;

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        state_view.active_receipt_rule_set = Some(model.rule_set_name);
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
        let mut results = vec![];
        if let Some(name) = view.active_receipt_rule_set.clone() {
            let arn = format!(
                "arn:aws:ses:{}:{}:receipt-rule-set/{}",
                ctx.default_region, ctx.default_account_id, name
            );
            let attrs = serde_json::json!({
                "id": name,
                "rule_set_name": name,
                "arn": arn,
            });
            results.push(ExtractedResource {
                name: name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ses_configuration_set
// ---------------------------------------------------------------------------

pub struct AwsSesConfigurationSetConverter {
    service: Arc<SesService>,
}

impl AwsSesConfigurationSetConverter {
    pub fn new(service: Arc<SesService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSesConfigurationSetConverter {
    fn resource_type(&self) -> &str {
        "aws_ses_configuration_set"
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

impl AwsSesConfigurationSetConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: sesv1_gen::ConfigurationSetTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ses_configuration_set", e))?;

        let cs_view = ConfigurationSetView {
            name: model.name.clone(),
            event_destinations: vec![],
            created_at: Utc::now(),
        };

        let mut state_view = SesV1StateView::default();
        state_view
            .configuration_sets
            .insert(model.name.clone(), cs_view);
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
            let arn = format!(
                "arn:aws:ses:{}:{}:configuration-set/{}",
                ctx.default_region, ctx.default_account_id, cs.name
            );
            let attrs = serde_json::json!({
                "id": cs.name,
                "name": cs.name,
                "arn": arn,
                "reputation_metrics_enabled": false,
                "sending_enabled": true,
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
// aws_ses_domain_dkim
// ---------------------------------------------------------------------------

pub struct AwsSesDomainDkimConverter {
    service: Arc<SesService>,
}

impl AwsSesDomainDkimConverter {
    pub fn new(service: Arc<SesService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSesDomainDkimConverter {
    fn resource_type(&self) -> &str {
        "aws_ses_domain_dkim"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ses_domain_identity"]
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

impl AwsSesDomainDkimConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: sesv1_gen::DomainDkimTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ses_domain_dkim", e))?;

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(identity) = state_view.identities.get_mut(&model.domain) {
            identity.dkim_enabled = true;
            if identity.dkim_tokens.is_empty() {
                // Synthesise three placeholder tokens to match the AWS
                // DKIM response shape.
                identity.dkim_tokens = vec![
                    format!("{}-dkim-token-1", model.domain),
                    format!("{}-dkim-token-2", model.domain),
                    format!("{}-dkim-token-3", model.domain),
                ];
            }
        } else {
            warnings.push(format!(
                "aws_ses_domain_dkim: domain identity '{}' not found in state; DKIM flag skipped",
                model.domain
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
            if identity.identity_type != "Domain" || !identity.dkim_enabled {
                continue;
            }
            let attrs = serde_json::json!({
                "id": identity.name,
                "domain": identity.name,
                "dkim_tokens": identity.dkim_tokens,
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
// aws_ses_domain_identity_verification
// ---------------------------------------------------------------------------

pub struct AwsSesDomainIdentityVerificationConverter {
    service: Arc<SesService>,
}

impl AwsSesDomainIdentityVerificationConverter {
    pub fn new(service: Arc<SesService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSesDomainIdentityVerificationConverter {
    fn resource_type(&self) -> &str {
        "aws_ses_domain_identity_verification"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ses_domain_identity"]
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

impl AwsSesDomainIdentityVerificationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: sesv1_gen::DomainIdentityVerificationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_ses_domain_identity_verification", e)
            })?;

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(identity) = state_view.identities.get_mut(&model.domain) {
            identity.verification_status = "Success".to_string();
        } else {
            warnings.push(format!(
                "aws_ses_domain_identity_verification: domain identity '{}' not found in state; verification skipped",
                model.domain
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
            if identity.identity_type != "Domain" || identity.verification_status != "Success" {
                continue;
            }
            let arn = format!(
                "arn:aws:ses:{}:{}:identity/{}",
                ctx.default_region, ctx.default_account_id, identity.name
            );
            let attrs = serde_json::json!({
                "id": identity.name,
                "domain": identity.name,
                "arn": arn,
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
// aws_ses_domain_mail_from
// ---------------------------------------------------------------------------

pub struct AwsSesDomainMailFromConverter {
    service: Arc<SesService>,
}

impl AwsSesDomainMailFromConverter {
    pub fn new(service: Arc<SesService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSesDomainMailFromConverter {
    fn resource_type(&self) -> &str {
        "aws_ses_domain_mail_from"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ses_domain_identity"]
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

impl AwsSesDomainMailFromConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: sesv1_gen::DomainMailFromTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ses_domain_mail_from", e))?;

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(identity) = state_view.identities.get_mut(&model.domain) {
            identity.mail_from_domain = Some(model.mail_from_domain.clone());
        } else {
            warnings.push(format!(
                "aws_ses_domain_mail_from: domain identity '{}' not found in state; mail-from skipped",
                model.domain
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
            if identity.identity_type != "Domain" {
                continue;
            }
            let Some(mail_from) = identity.mail_from_domain.clone() else {
                continue;
            };
            let attrs = serde_json::json!({
                "id": identity.name,
                "domain": identity.name,
                "mail_from_domain": mail_from,
                "behavior_on_mx_failure": "UseDefaultValue",
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
// aws_ses_event_destination
// ---------------------------------------------------------------------------

pub struct AwsSesEventDestinationConverter {
    service: Arc<SesService>,
}

impl AwsSesEventDestinationConverter {
    pub fn new(service: Arc<SesService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSesEventDestinationConverter {
    fn resource_type(&self) -> &str {
        "aws_ses_event_destination"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ses_configuration_set"]
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

impl AwsSesEventDestinationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: sesv1_gen::EventDestinationTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ses_event_destination", e))?;

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(cs) = state_view
            .configuration_sets
            .get_mut(&model.configuration_set_name)
        {
            cs.event_destinations.retain(|d| d.name != model.name);
            cs.event_destinations.push(EventDestinationView {
                name: model.name.clone(),
                enabled: model.enabled,
                matching_event_types: vec![],
            });
        } else {
            warnings.push(format!(
                "aws_ses_event_destination: configuration set '{}' not found in state; event destination '{}' skipped",
                model.configuration_set_name, model.name
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
            for dest in &cs.event_destinations {
                let arn = format!(
                    "arn:aws:ses:{}:{}:configuration-set/{}:event-destination/{}",
                    ctx.default_region, ctx.default_account_id, cs.name, dest.name
                );
                let attrs = serde_json::json!({
                    "id": dest.name,
                    "name": dest.name,
                    "configuration_set_name": cs.name,
                    "enabled": dest.enabled,
                    "arn": arn,
                });
                results.push(ExtractedResource {
                    name: dest.name.clone(),
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
// aws_ses_identity_notification_topic
// ---------------------------------------------------------------------------

pub struct AwsSesIdentityNotificationTopicConverter {
    service: Arc<SesService>,
}

impl AwsSesIdentityNotificationTopicConverter {
    pub fn new(service: Arc<SesService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSesIdentityNotificationTopicConverter {
    fn resource_type(&self) -> &str {
        "aws_ses_identity_notification_topic"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ses_domain_identity", "aws_ses_email_identity"]
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

impl AwsSesIdentityNotificationTopicConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: sesv1_gen::IdentityNotificationTopicTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_ses_identity_notification_topic", e)
            })?;

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(identity) = state_view.identities.get_mut(&model.identity) {
            match model.notification_type.as_str() {
                "Bounce" => identity.bounce_topic = model.topic_arn.clone(),
                "Complaint" => identity.complaint_topic = model.topic_arn.clone(),
                "Delivery" => identity.delivery_topic = model.topic_arn.clone(),
                other => warnings.push(format!(
                    "aws_ses_identity_notification_topic: unknown notification_type '{}'",
                    other
                )),
            }
        } else {
            warnings.push(format!(
                "aws_ses_identity_notification_topic: identity '{}' not found in state; topic skipped",
                model.identity
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
            for (notification_type, topic) in [
                ("Bounce", identity.bounce_topic.as_ref()),
                ("Complaint", identity.complaint_topic.as_ref()),
                ("Delivery", identity.delivery_topic.as_ref()),
            ] {
                let Some(topic_arn) = topic else { continue };
                let attrs = serde_json::json!({
                    "id": format!("{}|{}", identity.name, notification_type),
                    "identity": identity.name,
                    "notification_type": notification_type,
                    "topic_arn": topic_arn,
                    "include_original_headers": false,
                });
                results.push(ExtractedResource {
                    name: format!("{}-{}", identity.name, notification_type.to_lowercase()),
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
// aws_ses_identity_policy
// ---------------------------------------------------------------------------
// Warning-only: `SesV1StateView` does not model identity policies. The
// model is parsed for schema validation and a warning is emitted.

pub struct AwsSesIdentityPolicyConverter;

impl AwsSesIdentityPolicyConverter {
    pub fn new(_service: Arc<SesService>) -> Self {
        Self
    }
}

impl TerraformResourceConverter for AwsSesIdentityPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_ses_identity_policy"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let _model: sesv1_gen::IdentityPolicyTfModel =
                serde_json::from_value(instance.attributes.clone())
                    .map_err(|e| classify_deserialize_error("aws_ses_identity_policy", e))?;
            let msg = "aws_ses_identity_policy: identity policies are not modelled in SesV1StateView; resource skipped";
            eprintln!("warning: {}", msg);
            Ok(ConversionResult {
                region,
                warnings: vec![msg.to_string()],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

// ---------------------------------------------------------------------------
// aws_ses_receipt_filter
// ---------------------------------------------------------------------------
// Warning-only: `SesV1StateView` does not model receipt IP filters.

pub struct AwsSesReceiptFilterConverter;

impl AwsSesReceiptFilterConverter {
    pub fn new(_service: Arc<SesService>) -> Self {
        Self
    }
}

impl TerraformResourceConverter for AwsSesReceiptFilterConverter {
    fn resource_type(&self) -> &str {
        "aws_ses_receipt_filter"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let _model: sesv1_gen::ReceiptFilterTfModel =
                serde_json::from_value(instance.attributes.clone())
                    .map_err(|e| classify_deserialize_error("aws_ses_receipt_filter", e))?;
            let msg = "aws_ses_receipt_filter: receipt filters are not modelled in SesV1StateView; resource skipped";
            eprintln!("warning: {}", msg);
            Ok(ConversionResult {
                region,
                warnings: vec![msg.to_string()],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

// ---------------------------------------------------------------------------
// aws_ses_receipt_rule
// ---------------------------------------------------------------------------

pub struct AwsSesReceiptRuleConverter {
    service: Arc<SesService>,
}

impl AwsSesReceiptRuleConverter {
    pub fn new(service: Arc<SesService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSesReceiptRuleConverter {
    fn resource_type(&self) -> &str {
        "aws_ses_receipt_rule"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ses_receipt_rule_set"]
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

impl AwsSesReceiptRuleConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: sesv1_gen::ReceiptRuleTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ses_receipt_rule", e))?;

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        let rule_view = ReceiptRuleView {
            name: model.name.clone(),
            enabled: model.enabled,
            scan_enabled: model.scan_enabled,
            tls_policy: model.tls_policy.clone(),
        };
        if let Some(rule_set) = state_view.receipt_rule_sets.get_mut(&model.rule_set_name) {
            rule_set.rules.retain(|r| r.name != model.name);
            rule_set.rules.push(rule_view);
        } else {
            warnings.push(format!(
                "aws_ses_receipt_rule: receipt rule set '{}' not found in state; rule '{}' skipped",
                model.rule_set_name, model.name
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
        for rule_set in view.receipt_rule_sets.values() {
            for rule in &rule_set.rules {
                let attrs = serde_json::json!({
                    "id": rule.name,
                    "name": rule.name,
                    "rule_set_name": rule_set.name,
                    "enabled": rule.enabled,
                    "scan_enabled": rule.scan_enabled,
                    "tls_policy": rule.tls_policy,
                });
                results.push(ExtractedResource {
                    name: rule.name.clone(),
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
// aws_ses_receipt_rule_set
// ---------------------------------------------------------------------------

pub struct AwsSesReceiptRuleSetConverter {
    service: Arc<SesService>,
}

impl AwsSesReceiptRuleSetConverter {
    pub fn new(service: Arc<SesService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSesReceiptRuleSetConverter {
    fn resource_type(&self) -> &str {
        "aws_ses_receipt_rule_set"
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

impl AwsSesReceiptRuleSetConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: sesv1_gen::ReceiptRuleSetTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ses_receipt_rule_set", e))?;

        let rule_set_view = ReceiptRuleSetView {
            name: model.rule_set_name.clone(),
            rules: vec![],
            created_at: Utc::now(),
        };

        let mut state_view = SesV1StateView::default();
        state_view
            .receipt_rule_sets
            .insert(model.rule_set_name.clone(), rule_set_view);
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
        for rule_set in view.receipt_rule_sets.values() {
            let arn = format!(
                "arn:aws:ses:{}:{}:receipt-rule-set/{}",
                ctx.default_region, ctx.default_account_id, rule_set.name
            );
            let attrs = serde_json::json!({
                "id": rule_set.name,
                "rule_set_name": rule_set.name,
                "arn": arn,
            });
            results.push(ExtractedResource {
                name: rule_set.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ses_template
// ---------------------------------------------------------------------------

pub struct AwsSesTemplateConverter {
    service: Arc<SesService>,
}

impl AwsSesTemplateConverter {
    pub fn new(service: Arc<SesService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSesTemplateConverter {
    fn resource_type(&self) -> &str {
        "aws_ses_template"
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

impl AwsSesTemplateConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: sesv1_gen::TemplateTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_ses_template", e))?;

        let template_view = TemplateView {
            name: model.name.clone(),
            subject_part: model.subject.clone(),
            html_part: model.html.clone(),
            text_part: model.text.clone(),
            created_at: Utc::now(),
        };

        let mut state_view = SesV1StateView::default();
        state_view
            .templates
            .insert(model.name.clone(), template_view);
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
        for template in view.templates.values() {
            let arn = format!(
                "arn:aws:ses:{}:{}:template/{}",
                ctx.default_region, ctx.default_account_id, template.name
            );
            let attrs = serde_json::json!({
                "id": template.name,
                "name": template.name,
                "subject": template.subject_part,
                "html": template.html_part,
                "text": template.text_part,
                "arn": arn,
            });
            results.push(ExtractedResource {
                name: template.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

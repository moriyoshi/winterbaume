//! Terraform converters for SES v1 (2010-12-01) resources.
//!
//! `EmailIdentityTfModel` and `DomainIdentityTfModel` are generated
//! from `specs/sesv1.toml`. The ARN templates and the shared
//! `IdentityView` constant fields (verification status, DKIM tokens,
//! forwarding) are wired up here.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_ses::SesService;
use winterbaume_ses::views::{IdentityView, SesV1StateView};
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

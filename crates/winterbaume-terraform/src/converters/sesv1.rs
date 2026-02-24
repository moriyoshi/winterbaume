//! Terraform converters for SES v1 (2010-12-01) resources.

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
use crate::util::{extract_region, optional_str, require_str};

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
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let email = require_str(attrs, "email", "aws_ses_email_identity")?;
        let _arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:ses:{}:{}:identity/{}",
                region, ctx.default_account_id, email
            )
        });

        let identity_view = IdentityView {
            name: email.to_string(),
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
        state_view
            .identities
            .insert(email.to_string(), identity_view);
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
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let domain = require_str(attrs, "domain", "aws_ses_domain_identity")?;

        let arn = format!(
            "arn:aws:ses:{}:{}:identity/{}",
            region, ctx.default_account_id, domain
        );

        let identity_view = IdentityView {
            name: domain.to_string(),
            identity_type: "Domain".to_string(),
            verification_status: "Success".to_string(),
            verification_token: Some(
                attrs
                    .get("verification_token")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string(),
            ),
            dkim_tokens: vec![],
            dkim_enabled: false,
            mail_from_domain: None,
            bounce_topic: None,
            complaint_topic: None,
            delivery_topic: None,
            forwarding_enabled: true,
        };

        let mut state_view = SesV1StateView::default();
        state_view
            .identities
            .insert(domain.to_string(), identity_view);
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

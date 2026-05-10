//! Terraform converter for Signer resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_signer::SignerService;
use winterbaume_signer::views::{SignerStateView, SigningProfileView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::signer as signer_gen;
use crate::util::{classify_deserialize_error, extract_region, extract_tags};

// ---------------------------------------------------------------------------
// aws_signer_signing_profile
// ---------------------------------------------------------------------------

/// Converts `aws_signer_signing_profile` Terraform resources to/from Signer state.
pub struct AwsSignerSigningProfileConverter {
    service: Arc<SignerService>,
}

impl AwsSignerSigningProfileConverter {
    pub fn new(service: Arc<SignerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSignerSigningProfileConverter {
    fn resource_type(&self) -> &str {
        "aws_signer_signing_profile"
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

impl AwsSignerSigningProfileConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: signer_gen::SigningProfileTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_signer_signing_profile", e))?;

        let profile_name = model.name.clone();
        let platform_id = model.platform_id.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:signer:{}:{}:/signing-profiles/{}",
                region, ctx.default_account_id, profile_name
            )
        });
        let profile_version = model.version.unwrap_or_else(|| "1".to_string());
        let profile_version_arn = model
            .version_arn
            .unwrap_or_else(|| format!("{}/{}", arn, profile_version));
        let status = model.status.unwrap_or_else(|| "Active".to_string());
        let revision_id = model.revision_id.unwrap_or_default();
        let tags = extract_tags(&instance.attributes);

        // Parse `signature_validity_period` nested block: [{type: ..., value: ...}]
        let signature_validity_period: Option<serde_json::Value> = instance
            .attributes
            .get("signature_validity_period")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .map(|b| {
                serde_json::json!({
                    "type": b.get("type").and_then(|v| v.as_str()).unwrap_or("DAYS"),
                    "value": b.get("value").and_then(|v| v.as_i64()).unwrap_or(135),
                })
            });

        // Parse `signing_material` nested block: [{certificate_arn: ...}]
        let signing_material_certificate_arn: Option<String> = instance
            .attributes
            .get("signing_material")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|b| b.get("certificate_arn"))
            .and_then(|v| v.as_str())
            .map(String::from);

        let profile_view = SigningProfileView {
            profile_name: profile_name.clone(),
            profile_version,
            profile_version_arn,
            platform_id,
            status,
            arn,
            tags,
            permissions: vec![],
            revision_id,
            signature_validity_period,
            signing_material_certificate_arn,
        };

        let mut state_view = SignerStateView {
            profiles: HashMap::new(),
            jobs: HashMap::new(),
        };
        state_view.profiles.insert(profile_name, profile_view);
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
        for profile in view.profiles.values() {
            let svp_block: Vec<serde_json::Value> =
                if let Some(svp) = &profile.signature_validity_period {
                    vec![svp.clone()]
                } else {
                    vec![]
                };

            let signing_material_block: Vec<serde_json::Value> = profile
                .signing_material_certificate_arn
                .as_deref()
                .map(|arn| vec![serde_json::json!({"certificate_arn": arn})])
                .unwrap_or_default();

            let attrs = serde_json::json!({
                "id": profile.profile_name,
                "name": profile.profile_name,
                "version": profile.profile_version,
                "version_arn": profile.profile_version_arn,
                "platform_id": profile.platform_id,
                "status": profile.status,
                "arn": profile.arn,
                "tags": profile.tags,
                "revision_id": profile.revision_id,
                "signature_validity_period": svp_block,
                "signing_material": signing_material_block,
            });
            results.push(ExtractedResource {
                name: profile.profile_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

//! Terraform converter for `aws_acm_certificate` resources.
//!
//! `CertificateTfModel` is generated from `specs/acm.toml`. The ARN
//! template (with the `id` fallback chain), the synthesised UUID body
//! when neither is supplied, the `subject_alternative_names` /
//! `options` / `validation_option` nested-array reads, and the merged
//! `tags` / `tags_all` projection into `Vec<TagView>` are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_acm::AcmService;
use winterbaume_acm::views::{
    AcmStateView, CertificateOptionsView, CertificateView, DomainValidationOptionView, TagView,
};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::acm as acm_gen;
use crate::util::{classify_deserialize_error, extract_region};

/// Converts `aws_acm_certificate` Terraform resources to/from ACM state.
pub struct AwsAcmCertificateConverter {
    service: Arc<AcmService>,
}

impl AwsAcmCertificateConverter {
    pub fn new(service: Arc<AcmService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAcmCertificateConverter {
    fn resource_type(&self) -> &str {
        "aws_acm_certificate"
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

impl AwsAcmCertificateConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: acm_gen::CertificateTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_acm_certificate", e))?;

        let attrs = &instance.attributes;
        let domain_name = model.domain_name.clone();

        let arn = model.arn.or(model.id).unwrap_or_else(|| {
            format!(
                "arn:aws:acm:{}:{}:certificate/{}",
                region,
                ctx.default_account_id,
                uuid::Uuid::new_v4()
            )
        });

        // SANs from subject_alternative_names list
        let sans: Vec<String> = attrs
            .get("subject_alternative_names")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_default();

        // Tags from tags / tags_all
        let tags: Vec<TagView> = {
            let mut map: HashMap<String, String> = HashMap::new();
            if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
                for (k, v) in obj {
                    if let Some(s) = v.as_str() {
                        map.insert(k.clone(), s.to_string());
                    }
                }
            }
            if let Some(obj) = attrs.get("tags").and_then(|v| v.as_object()) {
                for (k, v) in obj {
                    if let Some(s) = v.as_str() {
                        map.insert(k.clone(), s.to_string());
                    }
                }
            }
            map.into_iter()
                .map(|(k, v)| TagView {
                    key: k,
                    value: Some(v),
                })
                .collect()
        };

        let status = model.status.unwrap_or_else(|| "ISSUED".to_string());

        // Parse `options` block: [{"certificate_transparency_logging_preference": "ENABLED"}]
        let options = attrs
            .get("options")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .map(|block| {
                let pref = block
                    .get("certificate_transparency_logging_preference")
                    .and_then(|v| v.as_str())
                    .unwrap_or("ENABLED")
                    .to_string();
                CertificateOptionsView {
                    certificate_transparency_logging_preference: pref,
                }
            });

        // `validation_option` — parse and store in domain_validation_options so extract can round-trip them.
        let validation_options: Vec<(String, String)> = attrs
            .get("validation_option")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|b| {
                        let dn = b.get("domain_name").and_then(|v| v.as_str())?;
                        let vd = b.get("validation_domain").and_then(|v| v.as_str())?;
                        Some((dn.to_string(), vd.to_string()))
                    })
                    .collect()
            })
            .unwrap_or_default();

        let domain_validation_options: Vec<DomainValidationOptionView> = validation_options
            .into_iter()
            .map(|(dn, vd)| DomainValidationOptionView {
                domain_name: dn,
                validation_domain: vd,
                validation_status: "SUCCESS".to_string(),
            })
            .collect();

        let cert_view = CertificateView {
            arn: arn.clone(),
            domain_name,
            status,
            subject_alternative_names: sans,
            created_at: Some(Utc::now().to_rfc3339()),
            certificate_type: "AMAZON_ISSUED".to_string(),
            tags,
            issuer: "Amazon".to_string(),
            key_algorithm: model
                .key_algorithm
                .unwrap_or_else(|| "RSA_2048".to_string()),
            renewal_eligibility: "ELIGIBLE".to_string(),
            options: Some(options.unwrap_or(CertificateOptionsView {
                certificate_transparency_logging_preference: "ENABLED".to_string(),
            })),
            domain_validation_options,
            not_before: None,
            not_after: None,
            certificate_authority_arn: model.certificate_authority_arn,
            certificate_pem: None,
            certificate_chain: None,
            private_key: None,
        };

        let mut state_view = AcmStateView::default();
        state_view.certificates.insert(arn, cert_view);
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
        for cert in view.certificates.values() {
            let tags: HashMap<String, String> = cert
                .tags
                .iter()
                .filter_map(|t| t.value.as_ref().map(|v| (t.key.clone(), v.clone())))
                .collect();

            let options_block: Vec<serde_json::Value> = cert
                .options
                .as_ref()
                .map(|o| {
                    vec![serde_json::json!({
                        "certificate_transparency_logging_preference":
                            o.certificate_transparency_logging_preference,
                    })]
                })
                .unwrap_or_default();

            let domain_validation: Vec<serde_json::Value> = cert
                .domain_validation_options
                .iter()
                .map(|dvo| {
                    serde_json::json!({
                        "domain_name": dvo.domain_name,
                        "validation_domain": dvo.validation_domain,
                        "validation_status": dvo.validation_status,
                    })
                })
                .collect();

            // Emit validation_option blocks so Terraform can round-trip them.
            let validation_option_block: Vec<serde_json::Value> = cert
                .domain_validation_options
                .iter()
                .map(|dvo| {
                    serde_json::json!({
                        "domain_name": dvo.domain_name,
                        "validation_domain": dvo.validation_domain,
                    })
                })
                .collect();

            let attrs = serde_json::json!({
                "id": cert.arn,
                "arn": cert.arn,
                "domain_name": cert.domain_name,
                "subject_alternative_names": cert.subject_alternative_names,
                "status": cert.status,
                "certificate_type": cert.certificate_type,
                "issuer": cert.issuer,
                "key_algorithm": cert.key_algorithm,
                "not_before": cert.not_before,
                "not_after": cert.not_after,
                "domain_validation_options": domain_validation,
                "validation_option": validation_option_block,
                "renewal_eligibility": cert.renewal_eligibility,
                "tags": tags,
                "tags_all": tags,
                "validation_method": "DNS",
                "options": options_block,
            });
            results.push(ExtractedResource {
                name: cert.domain_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

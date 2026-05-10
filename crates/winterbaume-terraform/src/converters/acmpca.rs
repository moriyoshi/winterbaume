//! Terraform converter for ACM PCA resources.
//!
//! `CertificateAuthorityTfModel` is generated from
//! `specs/acmpca.toml`. ARN templates, the synthesised
//! certificate-authority UUID, the constants (`type = "SUBORDINATE"`,
//! `key_storage_security_standard = "FIPS_140_2_LEVEL_3_OR_HIGHER"`,
//! `status = "ACTIVE"`), the nested
//! `certificate_authority_configuration` block (key/signing
//! algorithm, subject), and the nested `revocation_configuration`
//! block are all wired up here from the raw attributes.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_acmpca::AcmPcaService;
use winterbaume_acmpca::views::{
    AcmPcaStateView, CaConfigurationView, CaSubjectView, CertificateAuthorityView,
    CrlConfigurationView, RevocationConfigurationView, TagView,
};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::acmpca as acmpca_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_acmpca_certificate_authority
// ---------------------------------------------------------------------------

/// Converts `aws_acmpca_certificate_authority` Terraform resources to/from
/// ACM PCA state.
pub struct AwsAcmpcaCertificateAuthorityConverter {
    service: Arc<AcmPcaService>,
}

impl AwsAcmpcaCertificateAuthorityConverter {
    pub fn new(service: Arc<AcmPcaService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAcmpcaCertificateAuthorityConverter {
    fn resource_type(&self) -> &str {
        "aws_acmpca_certificate_authority"
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

impl AwsAcmpcaCertificateAuthorityConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: acmpca_gen::CertificateAuthorityTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_acmpca_certificate_authority", e))?;

        let attrs = &instance.attributes;

        let ca_type = model.r#type.unwrap_or_else(|| "SUBORDINATE".to_string());
        let key_storage_security_standard = model
            .key_storage_security_standard
            .unwrap_or_else(|| "FIPS_140_2_LEVEL_3_OR_HIGHER".to_string());

        // Nested block: certificate_authority_configuration.
        let ca_config_val = attrs.get("certificate_authority_configuration");
        let key_algorithm = ca_config_val
            .and_then(|c| c.get("key_algorithm"))
            .and_then(|v| v.as_str())
            .unwrap_or("RSA_2048")
            .to_string();
        let signing_algorithm = ca_config_val
            .and_then(|c| c.get("signing_algorithm"))
            .and_then(|v| v.as_str())
            .unwrap_or("SHA256WITHRSA")
            .to_string();

        let subject_val = ca_config_val.and_then(|c| c.get("subject"));
        let common_name = subject_val
            .and_then(|s| s.get("common_name"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let country = subject_val
            .and_then(|s| s.get("country"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let state = subject_val
            .and_then(|s| s.get("state"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let organization = subject_val
            .and_then(|s| s.get("organization"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let organizational_unit = subject_val
            .and_then(|s| s.get("organizational_unit"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let locality = subject_val
            .and_then(|s| s.get("locality"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // Nested block: revocation_configuration.
        let rev_config_val = attrs.get("revocation_configuration");
        let revocation_configuration = rev_config_val.map(|rc| {
            let crl_val = rc.get("crl_configuration");
            RevocationConfigurationView {
                crl_configuration: crl_val.map(|crl| CrlConfigurationView {
                    enabled: crl
                        .get("enabled")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false),
                    s3_object_acl: crl
                        .get("s3_object_acl")
                        .and_then(|v| v.as_str())
                        .unwrap_or("BUCKET_OWNER_FULL_CONTROL")
                        .to_string(),
                }),
            }
        });

        // tags HashMap -> Vec<TagView>.
        let tags: Vec<TagView> = model
            .tags
            .into_iter()
            .map(|(k, v)| TagView { key: k, value: v })
            .collect();

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:acm-pca:{}:{}:certificate-authority/{}",
                region,
                ctx.default_account_id,
                uuid::Uuid::new_v4()
            )
        });

        let ca_view = CertificateAuthorityView {
            arn: arn.clone(),
            owner_account: ctx.default_account_id.clone(),
            ca_type,
            status: model.status.unwrap_or_else(|| "ACTIVE".to_string()),
            created_at: chrono::Utc::now().to_rfc3339(),
            last_state_change_at: None,
            not_before: None,
            not_after: None,
            ca_config: CaConfigurationView {
                key_algorithm,
                signing_algorithm,
                subject: CaSubjectView {
                    common_name,
                    country,
                    state,
                    organization,
                    organizational_unit,
                    locality,
                },
            },
            key_storage_security_standard,
            revocation_configuration,
            tags,
            private_key_pem: String::new(),
            csr_pem: String::new(),
            certificate_pem: model.certificate,
            certificate_chain_pem: model.certificate_chain,
            issued_certificates: HashMap::new(),
            revoked_certificates: HashMap::new(),
            policy: model.policy,
            permissions: HashMap::new(),
            audit_reports: HashMap::new(),
        };

        let state_view = AcmPcaStateView {
            certificate_authorities: {
                let mut m = HashMap::new();
                m.insert(arn, ca_view);
                m
            },
        };
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
        for ca in view.certificate_authorities.values() {
            let tags_map: HashMap<String, String> = ca
                .tags
                .iter()
                .map(|t| (t.key.clone(), t.value.clone()))
                .collect();
            let attrs = serde_json::json!({
                "id": ca.arn,
                "arn": ca.arn,
                "type": ca.ca_type,
                "status": ca.status,
                "created_at": ca.created_at,
                "key_storage_security_standard": ca.key_storage_security_standard,
                "certificate_authority_configuration": {
                    "key_algorithm": ca.ca_config.key_algorithm,
                    "signing_algorithm": ca.ca_config.signing_algorithm,
                    "subject": {
                        "common_name": ca.ca_config.subject.common_name,
                        "country": ca.ca_config.subject.country,
                        "state": ca.ca_config.subject.state,
                        "organization": ca.ca_config.subject.organization,
                        "organizational_unit": ca.ca_config.subject.organizational_unit,
                        "locality": ca.ca_config.subject.locality,
                    },
                },
                "revocation_configuration": ca.revocation_configuration.as_ref().map(|rc| {
                    serde_json::json!({
                        "crl_configuration": rc.crl_configuration.as_ref().map(|crl| {
                            serde_json::json!({
                                "enabled": crl.enabled,
                                "s3_object_acl": crl.s3_object_acl,
                            })
                        }),
                    })
                }),
                "certificate": ca.certificate_pem,
                "certificate_chain": ca.certificate_chain_pem,
                "not_before": ca.not_before,
                "not_after": ca.not_after,
                "tags": tags_map,
            });
            results.push(ExtractedResource {
                name: ca.arn.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

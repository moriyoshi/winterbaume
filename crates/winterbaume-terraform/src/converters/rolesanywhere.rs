//! Terraform converters for IAM Roles Anywhere resources.
//!
//! `ProfileTfModel` and `TrustAnchorTfModel` are generated from
//! `specs/rolesanywhere.toml`. The ARN templates, the synthesised UUID
//! identifiers, the `role_arns` / `managed_policy_arns` Vec<String>
//! fields, the `Option<i64>` / `Option<bool>` fields, and the nested
//! `source` block are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_rolesanywhere::RolesAnywhereService;
use winterbaume_rolesanywhere::views::{
    ProfileView, RolesAnywhereStateView, SourceDataView, TrustAnchorView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::rolesanywhere as rolesanywhere_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_rolesanywhere_profile
// ---------------------------------------------------------------------------

pub struct AwsRolesAnywhereProfileConverter {
    service: Arc<RolesAnywhereService>,
}

impl AwsRolesAnywhereProfileConverter {
    pub fn new(service: Arc<RolesAnywhereService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRolesAnywhereProfileConverter {
    fn resource_type(&self) -> &str {
        "aws_rolesanywhere_profile"
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

impl AwsRolesAnywhereProfileConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: rolesanywhere_gen::ProfileTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_rolesanywhere_profile", e))?;

        let duration_seconds = attrs
            .get("duration_seconds")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);
        let require_instance_properties = attrs
            .get("require_instance_properties")
            .and_then(|v| v.as_bool());
        let accept_role_session_name = attrs
            .get("accept_role_session_name")
            .and_then(|v| v.as_bool());

        let role_arns: Vec<String> = attrs
            .get("role_arns")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let managed_policy_arns: Vec<String> = attrs
            .get("managed_policy_arns")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let profile_id = model.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let profile_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rolesanywhere:{}:{}:profile/{}",
                region, ctx.default_account_id, profile_id
            )
        });

        let mut profiles = HashMap::new();
        profiles.insert(
            profile_id.clone(),
            ProfileView {
                profile_id,
                profile_arn,
                name: model.name,
                enabled: model.enabled,
                role_arns,
                managed_policy_arns,
                session_policy: model.session_policy,
                duration_seconds,
                require_instance_properties,
                accept_role_session_name,
                attribute_mappings: vec![],
                created_by: None,
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
                tags: model.tags,
            },
        );

        let view = RolesAnywhereStateView {
            profiles,
            ..Default::default()
        };
        self.service
            .merge(&ctx.default_account_id, &region, view)
            .await
            .map_err(ConversionError::StateView)?;

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

        let mut results = Vec::new();
        for profile in view.profiles.values() {
            let mut attrs = serde_json::Map::new();
            attrs.insert("id".to_string(), serde_json::json!(profile.profile_id));
            attrs.insert("arn".to_string(), serde_json::json!(profile.profile_arn));
            attrs.insert("name".to_string(), serde_json::json!(profile.name));
            attrs.insert("enabled".to_string(), serde_json::json!(profile.enabled));
            attrs.insert(
                "role_arns".to_string(),
                serde_json::json!(profile.role_arns),
            );
            if !profile.managed_policy_arns.is_empty() {
                attrs.insert(
                    "managed_policy_arns".to_string(),
                    serde_json::json!(profile.managed_policy_arns),
                );
            }
            if let Some(ref sp) = profile.session_policy {
                attrs.insert("session_policy".to_string(), serde_json::json!(sp));
            }
            if let Some(ds) = profile.duration_seconds {
                attrs.insert("duration_seconds".to_string(), serde_json::json!(ds));
            }

            let tags_map: serde_json::Map<String, serde_json::Value> = profile
                .tags
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::json!(v)))
                .collect();
            attrs.insert("tags".to_string(), serde_json::Value::Object(tags_map));

            results.push(ExtractedResource {
                name: profile.name.replace(' ', "_").to_lowercase(),
                attributes: serde_json::Value::Object(attrs),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_rolesanywhere_trust_anchor
// ---------------------------------------------------------------------------

pub struct AwsRolesAnywhereTrustAnchorConverter {
    service: Arc<RolesAnywhereService>,
}

impl AwsRolesAnywhereTrustAnchorConverter {
    pub fn new(service: Arc<RolesAnywhereService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRolesAnywhereTrustAnchorConverter {
    fn resource_type(&self) -> &str {
        "aws_rolesanywhere_trust_anchor"
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

impl AwsRolesAnywhereTrustAnchorConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: rolesanywhere_gen::TrustAnchorTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_rolesanywhere_trust_anchor", e))?;

        let trust_anchor_id = model.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let trust_anchor_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rolesanywhere:{}:{}:trust-anchor/{}",
                region, ctx.default_account_id, trust_anchor_id
            )
        });

        // Parse source block
        let (source_type, source_data) =
            if let Some(source_arr) = attrs.get("source").and_then(|v| v.as_array()) {
                if let Some(source_obj) = source_arr.first().and_then(|v| v.as_object()) {
                    let st = source_obj
                        .get("source_type")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    let sd = source_obj
                        .get("source_data")
                        .and_then(|v| v.as_array())
                        .and_then(|arr| arr.first())
                        .and_then(|v| v.as_object())
                        .and_then(|sd_obj| {
                            if let Some(cert) =
                                sd_obj.get("x509_certificate_data").and_then(|v| v.as_str())
                            {
                                Some(SourceDataView::X509CertificateData(cert.to_string()))
                            } else {
                                sd_obj
                                    .get("acm_pca_arn")
                                    .and_then(|v| v.as_str())
                                    .map(|arn| SourceDataView::AcmPcaArn(arn.to_string()))
                            }
                        });
                    (st, sd)
                } else {
                    (None, None)
                }
            } else {
                (None, None)
            };

        let mut trust_anchors = HashMap::new();
        trust_anchors.insert(
            trust_anchor_id.clone(),
            TrustAnchorView {
                trust_anchor_id,
                trust_anchor_arn,
                name: model.name,
                source_type,
                source_data,
                enabled: model.enabled,
                notification_settings: vec![],
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
                tags: model.tags,
            },
        );

        let view = RolesAnywhereStateView {
            trust_anchors,
            ..Default::default()
        };
        self.service
            .merge(&ctx.default_account_id, &region, view)
            .await
            .map_err(ConversionError::StateView)?;

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

        let mut results = Vec::new();
        for ta in view.trust_anchors.values() {
            let mut attrs = serde_json::Map::new();
            attrs.insert("id".to_string(), serde_json::json!(ta.trust_anchor_id));
            attrs.insert("arn".to_string(), serde_json::json!(ta.trust_anchor_arn));
            attrs.insert("name".to_string(), serde_json::json!(ta.name));
            attrs.insert("enabled".to_string(), serde_json::json!(ta.enabled));

            let tags_map: serde_json::Map<String, serde_json::Value> = ta
                .tags
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::json!(v)))
                .collect();
            attrs.insert("tags".to_string(), serde_json::Value::Object(tags_map));

            results.push(ExtractedResource {
                name: ta.name.replace(' ', "_").to_lowercase(),
                attributes: serde_json::Value::Object(attrs),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
            });
        }
        Ok(results)
    }
}

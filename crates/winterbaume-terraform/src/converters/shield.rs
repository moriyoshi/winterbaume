//! Terraform converters for Shield resources.
//!
//! Most TF projection models are generated from `specs/shield.toml`. Five of
//! the seven Shield resource types have no representation on
//! [`winterbaume_shield::views::ShieldStateView`]: their converters parse the
//! TF payload, emit a warning, and leave state untouched (inject) /
//! return an empty list (extract). The remaining two ride on the existing
//! `subscription` and `protections` fields.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_core::StatefulService;
use winterbaume_shield::ShieldService;
use winterbaume_shield::views::{ProtectionView, ShieldStateView, SubscriptionView, TagView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::shield as shield_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_shield_protection
// ---------------------------------------------------------------------------

/// Converts `aws_shield_protection` Terraform resources to/from Shield state.
pub struct AwsShieldProtectionConverter {
    service: Arc<ShieldService>,
}

impl AwsShieldProtectionConverter {
    pub fn new(service: Arc<ShieldService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsShieldProtectionConverter {
    fn resource_type(&self) -> &str {
        "aws_shield_protection"
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

impl AwsShieldProtectionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: shield_gen::ProtectionTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_shield_protection", e))?;

        let protection_id = model.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let protection_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:shield::{}:protection/{}",
                ctx.default_account_id, protection_id
            )
        });

        // Convert tags from HashMap<String, String> to Vec<TagView>.
        let tags: Vec<TagView> = model
            .tags
            .into_iter()
            .map(|(k, v)| TagView { key: k, value: v })
            .collect();

        let protection_view = ProtectionView {
            id: protection_id.clone(),
            name: model.name,
            resource_arn: model.resource_arn,
            protection_arn,
            health_check_ids: vec![],
            tags,
        };

        let state_view = ShieldStateView {
            subscription: None,
            protections: {
                let mut m = HashMap::new();
                m.insert(protection_id, protection_view);
                m
            },
            tags: HashMap::new(),
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
        for prot in view.protections.values() {
            let tags_map: HashMap<String, String> = prot
                .tags
                .iter()
                .map(|t| (t.key.clone(), t.value.clone()))
                .collect();
            let attrs = serde_json::json!({
                "id": prot.id,
                "arn": prot.protection_arn,
                "name": prot.name,
                "resource_arn": prot.resource_arn,
                "protection_arn": prot.protection_arn,
                "health_check_ids": prot.health_check_ids,
                "tags": tags_map,
            });
            results.push(ExtractedResource {
                name: prot.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_shield_application_layer_automatic_response
// No state on ShieldStateView. Inject parses the payload and records a
// warning; extract is empty.
// ---------------------------------------------------------------------------

pub struct AwsShieldApplicationLayerAutomaticResponseConverter {
    #[allow(dead_code)]
    service: Arc<ShieldService>,
}

impl AwsShieldApplicationLayerAutomaticResponseConverter {
    pub fn new(service: Arc<ShieldService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsShieldApplicationLayerAutomaticResponseConverter {
    fn resource_type(&self) -> &str {
        "aws_shield_application_layer_automatic_response"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsShieldApplicationLayerAutomaticResponseConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: shield_gen::ApplicationLayerAutomaticResponseTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_shield_application_layer_automatic_response", e)
            })?;
        let warnings = vec![format!(
            "aws_shield_application_layer_automatic_response: no state representation for resource '{}'; skipped",
            model.resource_arn
        )];
        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_shield_drt_access_log_bucket_association
// No state representation.
// ---------------------------------------------------------------------------

pub struct AwsShieldDrtAccessLogBucketAssociationConverter {
    #[allow(dead_code)]
    service: Arc<ShieldService>,
}

impl AwsShieldDrtAccessLogBucketAssociationConverter {
    pub fn new(service: Arc<ShieldService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsShieldDrtAccessLogBucketAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_shield_drt_access_log_bucket_association"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsShieldDrtAccessLogBucketAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: shield_gen::DrtAccessLogBucketAssociationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_shield_drt_access_log_bucket_association", e)
            })?;
        let warnings = vec![format!(
            "aws_shield_drt_access_log_bucket_association: no state representation for bucket '{}'; skipped",
            model.log_bucket
        )];
        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_shield_drt_access_role_arn_association
// No state representation.
// ---------------------------------------------------------------------------

pub struct AwsShieldDrtAccessRoleArnAssociationConverter {
    #[allow(dead_code)]
    service: Arc<ShieldService>,
}

impl AwsShieldDrtAccessRoleArnAssociationConverter {
    pub fn new(service: Arc<ShieldService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsShieldDrtAccessRoleArnAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_shield_drt_access_role_arn_association"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsShieldDrtAccessRoleArnAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: shield_gen::DrtAccessRoleArnAssociationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_shield_drt_access_role_arn_association", e)
            })?;
        let warnings = vec![format!(
            "aws_shield_drt_access_role_arn_association: no state representation for role '{}'; skipped",
            model.role_arn
        )];
        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_shield_proactive_engagement
// No state representation.
// ---------------------------------------------------------------------------

pub struct AwsShieldProactiveEngagementConverter {
    #[allow(dead_code)]
    service: Arc<ShieldService>,
}

impl AwsShieldProactiveEngagementConverter {
    pub fn new(service: Arc<ShieldService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsShieldProactiveEngagementConverter {
    fn resource_type(&self) -> &str {
        "aws_shield_proactive_engagement"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsShieldProactiveEngagementConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: shield_gen::ProactiveEngagementTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_shield_proactive_engagement", e))?;
        let warnings = vec![format!(
            "aws_shield_proactive_engagement: no state representation (enabled={}); skipped",
            model.enabled
        )];
        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_shield_protection_group
// No state representation.
// ---------------------------------------------------------------------------

pub struct AwsShieldProtectionGroupConverter {
    #[allow(dead_code)]
    service: Arc<ShieldService>,
}

impl AwsShieldProtectionGroupConverter {
    pub fn new(service: Arc<ShieldService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsShieldProtectionGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_shield_protection_group"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsShieldProtectionGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: shield_gen::ProtectionGroupTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_shield_protection_group", e))?;
        let warnings = vec![format!(
            "aws_shield_protection_group: no state representation for group '{}' (aggregation={}, pattern={}); skipped",
            model.protection_group_id, model.aggregation, model.pattern
        )];
        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_shield_protection_health_check_association
// Rides on ProtectionView::health_check_ids in existing state.
// ---------------------------------------------------------------------------

pub struct AwsShieldProtectionHealthCheckAssociationConverter {
    service: Arc<ShieldService>,
}

impl AwsShieldProtectionHealthCheckAssociationConverter {
    pub fn new(service: Arc<ShieldService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsShieldProtectionHealthCheckAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_shield_protection_health_check_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_shield_protection"]
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

impl AwsShieldProtectionHealthCheckAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: shield_gen::ProtectionHealthCheckAssociationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_shield_protection_health_check_association", e)
            })?;

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(prot) = state_view.protections.get_mut(&model.shield_protection_id) {
            if !prot.health_check_ids.contains(&model.health_check_arn) {
                prot.health_check_ids.push(model.health_check_arn.clone());
            }
        } else {
            warnings.push(format!(
                "aws_shield_protection_health_check_association: protection '{}' not found in state; health check '{}' skipped",
                model.shield_protection_id, model.health_check_arn
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
        for prot in view.protections.values() {
            for hc_arn in &prot.health_check_ids {
                let id = format!("{}+{}", prot.id, hc_arn);
                let attrs = serde_json::json!({
                    "id": id,
                    "shield_protection_id": prot.id,
                    "health_check_arn": hc_arn,
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
// aws_shield_subscription
// Singleton: rides on the existing `ShieldStateView::subscription` field.
// ---------------------------------------------------------------------------

pub struct AwsShieldSubscriptionConverter {
    service: Arc<ShieldService>,
}

impl AwsShieldSubscriptionConverter {
    pub fn new(service: Arc<ShieldService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsShieldSubscriptionConverter {
    fn resource_type(&self) -> &str {
        "aws_shield_subscription"
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

impl AwsShieldSubscriptionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: shield_gen::SubscriptionTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_shield_subscription", e))?;

        let auto_renew = model.auto_renew.unwrap_or_else(|| "ENABLED".to_string());
        let subscription_arn = format!(
            "arn:aws:shield:{}:{}:subscription",
            region, ctx.default_account_id
        );
        let subscription_view = SubscriptionView {
            start_time: Utc::now(),
            end_time: None,
            time_commitment_in_seconds: 31_536_000,
            auto_renew,
            subscription_arn,
        };

        let state_view = ShieldStateView {
            subscription: Some(subscription_view),
            protections: HashMap::new(),
            tags: HashMap::new(),
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
        if let Some(sub) = view.subscription {
            let attrs = serde_json::json!({
                "id": sub.subscription_arn,
                "auto_renew": sub.auto_renew,
                "start_time": sub.start_time.to_rfc3339(),
                "end_time": sub.end_time.map(|t| t.to_rfc3339()),
                "time_commitment_in_seconds": sub.time_commitment_in_seconds,
                "subscription_arn": sub.subscription_arn,
            });
            results.push(ExtractedResource {
                name: "default".to_string(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

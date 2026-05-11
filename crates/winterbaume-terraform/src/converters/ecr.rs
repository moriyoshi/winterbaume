//! Terraform converter for ECR resources.
//!
//! `RepositoryTfModel` is generated from `specs/ecr.toml`. The ARN and
//! repository-URL templates, the `image_tag_mutability = "MUTABLE"`
//! default, the `created_at` timestamp, and the nested
//! `image_scanning_configuration` / `encryption_configuration` blocks are
//! wired up here.
//!
//! Additional registry-level and per-repository sub-resources are wired
//! up below. Because `EcrService::merge` only fans out repositories,
//! `registry_policy`, and `layer_uploads`, every converter whose target
//! lives in `account_settings`, `registry_scanning_configuration`,
//! `replication_configuration`, `pull_through_cache_rules`, or
//! `repository_creation_templates` performs a `snapshot` + mutate +
//! `restore` round-trip. This is documented inline at each call site.
//! The `aws_ecr_lifecycle_policy` and `aws_ecr_repository_policy`
//! converters likewise use snapshot/restore so that the per-repository
//! `lifecycle_policy` / `repository_policy` fields are not clobbered
//! by a merge that drops everything else on the repository.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_core::StatefulService;
use winterbaume_ecr::EcrService;
use winterbaume_ecr::views::{
    EcrStateView, EncryptionConfigView, ImageScanningConfigView, PullThroughCacheRuleView,
    RegistryScanningConfigView, RegistryScanningRuleView, ReplicationConfigView,
    ReplicationDestinationView, ReplicationRuleView, RepositoryCreationTemplateView,
    RepositoryFilterView, RepositoryView, ScanningRepositoryFilterView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::ecr as ecr_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_ecr_repository
// ---------------------------------------------------------------------------

/// Converts `aws_ecr_repository` Terraform resources to/from ECR state.
pub struct AwsEcrRepositoryConverter {
    service: Arc<EcrService>,
}

impl AwsEcrRepositoryConverter {
    pub fn new(service: Arc<EcrService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEcrRepositoryConverter {
    fn resource_type(&self) -> &str {
        "aws_ecr_repository"
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

impl AwsEcrRepositoryConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ecr_gen::RepositoryTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_ecr_repository", e))?;

        let name = model.name.clone();
        let arn = model.arn.or(model.id).unwrap_or_else(|| {
            format!(
                "arn:aws:ecr:{}:{}:repository/{}",
                region, ctx.default_account_id, name
            )
        });
        let repository_url = model.repository_url.unwrap_or_else(|| {
            format!(
                "{}.dkr.ecr.{}.amazonaws.com/{}",
                ctx.default_account_id, region, name
            )
        });
        let image_tag_mutability = model
            .image_tag_mutability
            .unwrap_or_else(|| "MUTABLE".to_string());

        // Nested blocks stay as raw reads — spec can't express list-of-object.
        let attrs = &instance.attributes;
        let scan_on_push = attrs
            .get("image_scanning_configuration")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|item| item.get("scan_on_push"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let encryption_configuration = attrs
            .get("encryption_configuration")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .map(|item| {
                let encryption_type = item
                    .get("encryption_type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("AES256")
                    .to_string();
                let kms_key = item
                    .get("kms_key")
                    .and_then(|v| v.as_str())
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string());
                EncryptionConfigView {
                    encryption_type,
                    kms_key,
                }
            })
            .unwrap_or_default();

        let repo_view = RepositoryView {
            repository_name: name.clone(),
            repository_arn: arn.clone(),
            repository_uri: repository_url,
            registry_id: ctx.default_account_id.clone(),
            created_at: Some(Utc::now().to_rfc3339()),
            images: vec![],
            tags: model.tags,
            lifecycle_policy: None,
            repository_policy: None,
            image_scanning_configuration: ImageScanningConfigView { scan_on_push },
            image_tag_mutability,
            encryption_configuration,
        };

        let state_view = EcrStateView {
            repositories: {
                let mut m = HashMap::new();
                m.insert(name, repo_view);
                m
            },
            registry_policy: None,
            registry_scanning_configuration: RegistryScanningConfigView::default(),
            replication_configuration: ReplicationConfigView::default(),
            pull_through_cache_rules: vec![],
            ..Default::default()
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
        for repo in view.repositories.values() {
            let mut encryption_obj = serde_json::json!({
                "encryption_type": repo.encryption_configuration.encryption_type,
            });
            if let Some(ref kms_key) = repo.encryption_configuration.kms_key {
                encryption_obj["kms_key"] = serde_json::json!(kms_key);
            }
            let attrs = serde_json::json!({
                "id": repo.repository_name,
                "name": repo.repository_name,
                "arn": repo.repository_arn,
                "repository_url": repo.repository_uri,
                "registry_id": repo.registry_id,
                "image_tag_mutability": repo.image_tag_mutability,
                "image_scanning_configuration": [{
                    "scan_on_push": repo.image_scanning_configuration.scan_on_push,
                }],
                "encryption_configuration": [encryption_obj],
                "tags": repo.tags,
            });
            results.push(ExtractedResource {
                name: repo.repository_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ecr_account_setting
// ---------------------------------------------------------------------------

/// Converts `aws_ecr_account_setting` Terraform resources to/from ECR state.
///
/// State-layer note: `EcrService::merge` does not fan out the
/// `account_settings` field, so this converter uses
/// `snapshot` + mutate + `restore` to preserve other state.
pub struct AwsEcrAccountSettingConverter {
    service: Arc<EcrService>,
}

impl AwsEcrAccountSettingConverter {
    pub fn new(service: Arc<EcrService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEcrAccountSettingConverter {
    fn resource_type(&self) -> &str {
        "aws_ecr_account_setting"
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

impl AwsEcrAccountSettingConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ecr_gen::AccountSettingTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ecr_account_setting", e))?;

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        state_view.account_settings.insert(model.name, model.value);
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
        for (name, value) in &view.account_settings {
            let attrs = serde_json::json!({
                "id": name,
                "name": name,
                "value": value,
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
// aws_ecr_lifecycle_policy
// ---------------------------------------------------------------------------

/// Converts `aws_ecr_lifecycle_policy` Terraform resources to/from ECR state.
///
/// State-layer note: lifecycle_policy is a field on
/// `RepositoryView`. `EcrService::merge` would overwrite the entire
/// repository entry, so this converter uses snapshot + mutate +
/// restore to keep the existing repository fields intact.
pub struct AwsEcrLifecyclePolicyConverter {
    service: Arc<EcrService>,
}

impl AwsEcrLifecyclePolicyConverter {
    pub fn new(service: Arc<EcrService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEcrLifecyclePolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_ecr_lifecycle_policy"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ecr_repository"]
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

impl AwsEcrLifecyclePolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ecr_gen::LifecyclePolicyTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ecr_lifecycle_policy", e))?;

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        match state_view.repositories.get_mut(&model.repository) {
            Some(repo) => {
                repo.lifecycle_policy = Some(model.policy);
            }
            None => {
                warnings.push(format!(
                    "aws_ecr_lifecycle_policy: repository {:?} not found; policy dropped",
                    model.repository
                ));
            }
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
        for repo in view.repositories.values() {
            let Some(policy) = &repo.lifecycle_policy else {
                continue;
            };
            let attrs = serde_json::json!({
                "id": repo.repository_name,
                "repository": repo.repository_name,
                "policy": policy,
                "registry_id": repo.registry_id,
            });
            results.push(ExtractedResource {
                name: repo.repository_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ecr_pull_through_cache_rule
// ---------------------------------------------------------------------------

/// Converts `aws_ecr_pull_through_cache_rule` Terraform resources to/from ECR state.
///
/// State-layer note: `EcrService::merge` does not fan out
/// `pull_through_cache_rules`, so this converter uses snapshot +
/// mutate + restore.
pub struct AwsEcrPullThroughCacheRuleConverter {
    service: Arc<EcrService>,
}

impl AwsEcrPullThroughCacheRuleConverter {
    pub fn new(service: Arc<EcrService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEcrPullThroughCacheRuleConverter {
    fn resource_type(&self) -> &str {
        "aws_ecr_pull_through_cache_rule"
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

impl AwsEcrPullThroughCacheRuleConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ecr_gen::PullThroughCacheRuleTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ecr_pull_through_cache_rule", e))?;

        let registry_id = model
            .registry_id
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let rule_view = PullThroughCacheRuleView {
            ecr_repository_prefix: model.ecr_repository_prefix.clone(),
            upstream_registry_url: model.upstream_registry_url,
            registry_id: Some(registry_id),
            created_at: Some(Utc::now().to_rfc3339()),
            upstream_registry: None,
            upstream_repository_prefix: model.upstream_repository_prefix,
            credential_arn: model.credential_arn,
            custom_role_arn: model.custom_role_arn,
        };

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        // Replace any existing rule with the same prefix; otherwise push.
        if let Some(slot) = state_view
            .pull_through_cache_rules
            .iter_mut()
            .find(|r| r.ecr_repository_prefix == model.ecr_repository_prefix)
        {
            *slot = rule_view;
        } else {
            state_view.pull_through_cache_rules.push(rule_view);
        }
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
        for rule in &view.pull_through_cache_rules {
            let attrs = serde_json::json!({
                "id": rule.ecr_repository_prefix,
                "ecr_repository_prefix": rule.ecr_repository_prefix,
                "upstream_registry_url": rule.upstream_registry_url,
                "credential_arn": rule.credential_arn,
                "upstream_repository_prefix": rule.upstream_repository_prefix,
                "custom_role_arn": rule.custom_role_arn,
                "registry_id": rule.registry_id,
            });
            results.push(ExtractedResource {
                name: rule.ecr_repository_prefix.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ecr_registry_policy
// ---------------------------------------------------------------------------

/// Converts `aws_ecr_registry_policy` Terraform resources to/from ECR state.
///
/// `EcrService::merge` does propagate `registry_policy` when present,
/// so this converter uses the merge path.
pub struct AwsEcrRegistryPolicyConverter {
    service: Arc<EcrService>,
}

impl AwsEcrRegistryPolicyConverter {
    pub fn new(service: Arc<EcrService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEcrRegistryPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_ecr_registry_policy"
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

impl AwsEcrRegistryPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ecr_gen::RegistryPolicyTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ecr_registry_policy", e))?;

        let state_view = EcrStateView {
            registry_policy: Some(model.policy),
            ..Default::default()
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
        let Some(policy) = view.registry_policy else {
            return Ok(vec![]);
        };
        let attrs = serde_json::json!({
            "id": ctx.default_account_id,
            "policy": policy,
            "registry_id": ctx.default_account_id,
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
// aws_ecr_registry_scanning_configuration
// ---------------------------------------------------------------------------

/// Converts `aws_ecr_registry_scanning_configuration` Terraform
/// resources to/from ECR state.
///
/// State-layer note: `EcrService::merge` does not propagate
/// `registry_scanning_configuration`, so snapshot + mutate + restore
/// is used. The `rule` block is a list-of-object that the spec format
/// cannot describe; it is read raw from `instance.attributes`.
pub struct AwsEcrRegistryScanningConfigurationConverter {
    service: Arc<EcrService>,
}

impl AwsEcrRegistryScanningConfigurationConverter {
    pub fn new(service: Arc<EcrService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEcrRegistryScanningConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_ecr_registry_scanning_configuration"
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

impl AwsEcrRegistryScanningConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ecr_gen::RegistryScanningConfigurationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_ecr_registry_scanning_configuration", e)
            })?;

        // Parse nested `rule { scan_frequency, repository_filter { filter, filter_type } }`.
        let rules: Vec<RegistryScanningRuleView> = instance
            .attributes
            .get("rule")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|rule| {
                        let scan_frequency = rule
                            .get("scan_frequency")
                            .and_then(|v| v.as_str())
                            .unwrap_or_default()
                            .to_string();
                        let repository_filters: Vec<ScanningRepositoryFilterView> = rule
                            .get("repository_filter")
                            .and_then(|v| v.as_array())
                            .map(|fs| {
                                fs.iter()
                                    .map(|f| ScanningRepositoryFilterView {
                                        filter: f
                                            .get("filter")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or_default()
                                            .to_string(),
                                        filter_type: f
                                            .get("filter_type")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("WILDCARD")
                                            .to_string(),
                                    })
                                    .collect()
                            })
                            .unwrap_or_default();
                        RegistryScanningRuleView {
                            scan_frequency,
                            repository_filters,
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        state_view.registry_scanning_configuration = RegistryScanningConfigView {
            scan_type: model.scan_type,
            rules,
        };
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
        let cfg = &view.registry_scanning_configuration;
        // Only emit a resource when scanning configuration looks
        // non-default (i.e. there is at least one rule, or scan_type
        // has been changed away from "BASIC"). Otherwise no Terraform
        // resource is present in state.
        if cfg.rules.is_empty() && cfg.scan_type == "BASIC" {
            return Ok(vec![]);
        }
        let rules: Vec<serde_json::Value> = cfg
            .rules
            .iter()
            .map(|r| {
                let filters: Vec<serde_json::Value> = r
                    .repository_filters
                    .iter()
                    .map(|f| {
                        serde_json::json!({
                            "filter": f.filter,
                            "filter_type": f.filter_type,
                        })
                    })
                    .collect();
                serde_json::json!({
                    "scan_frequency": r.scan_frequency,
                    "repository_filter": filters,
                })
            })
            .collect();
        let attrs = serde_json::json!({
            "id": ctx.default_account_id,
            "scan_type": cfg.scan_type,
            "rule": rules,
            "registry_id": ctx.default_account_id,
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
// aws_ecr_replication_configuration
// ---------------------------------------------------------------------------

/// Converts `aws_ecr_replication_configuration` Terraform resources to/from ECR state.
///
/// State-layer note: `EcrService::merge` does not propagate
/// `replication_configuration`, so this converter uses snapshot +
/// mutate + restore. The nested
/// `replication_configuration { rule { destination, repository_filter } }`
/// block is read raw from `instance.attributes`.
pub struct AwsEcrReplicationConfigurationConverter {
    service: Arc<EcrService>,
}

impl AwsEcrReplicationConfigurationConverter {
    pub fn new(service: Arc<EcrService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEcrReplicationConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_ecr_replication_configuration"
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

impl AwsEcrReplicationConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: ecr_gen::ReplicationConfigurationTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ecr_replication_configuration", e))?;

        // Find the outer `replication_configuration` block (list of one).
        let outer = instance
            .attributes
            .get("replication_configuration")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first());

        let rules: Vec<ReplicationRuleView> = outer
            .and_then(|o| o.get("rule"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|rule| {
                        let destinations: Vec<ReplicationDestinationView> = rule
                            .get("destination")
                            .and_then(|v| v.as_array())
                            .map(|ds| {
                                ds.iter()
                                    .map(|d| ReplicationDestinationView {
                                        region: d
                                            .get("region")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or_default()
                                            .to_string(),
                                        registry_id: d
                                            .get("registry_id")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or(&ctx.default_account_id)
                                            .to_string(),
                                    })
                                    .collect()
                            })
                            .unwrap_or_default();
                        let repository_filters: Vec<RepositoryFilterView> = rule
                            .get("repository_filter")
                            .and_then(|v| v.as_array())
                            .map(|fs| {
                                fs.iter()
                                    .map(|f| RepositoryFilterView {
                                        filter: f
                                            .get("filter")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or_default()
                                            .to_string(),
                                        filter_type: f
                                            .get("filter_type")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("PREFIX_MATCH")
                                            .to_string(),
                                    })
                                    .collect()
                            })
                            .unwrap_or_default();
                        ReplicationRuleView {
                            destinations,
                            repository_filters,
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        state_view.replication_configuration = ReplicationConfigView { rules };
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
        if view.replication_configuration.rules.is_empty() {
            return Ok(vec![]);
        }
        let rules: Vec<serde_json::Value> = view
            .replication_configuration
            .rules
            .iter()
            .map(|r| {
                let destinations: Vec<serde_json::Value> = r
                    .destinations
                    .iter()
                    .map(|d| {
                        serde_json::json!({
                            "region": d.region,
                            "registry_id": d.registry_id,
                        })
                    })
                    .collect();
                let filters: Vec<serde_json::Value> = r
                    .repository_filters
                    .iter()
                    .map(|f| {
                        serde_json::json!({
                            "filter": f.filter,
                            "filter_type": f.filter_type,
                        })
                    })
                    .collect();
                serde_json::json!({
                    "destination": destinations,
                    "repository_filter": filters,
                })
            })
            .collect();
        let attrs = serde_json::json!({
            "id": ctx.default_account_id,
            "registry_id": ctx.default_account_id,
            "replication_configuration": [{
                "rule": rules,
            }],
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
// aws_ecr_repository_creation_template
// ---------------------------------------------------------------------------

/// Converts `aws_ecr_repository_creation_template` Terraform resources
/// to/from ECR state.
///
/// State-layer note: `EcrService::merge` does not propagate
/// `repository_creation_templates`, so this converter uses snapshot +
/// mutate + restore. The nested `encryption_configuration` and
/// `resource_tags` blocks plus the `applied_for` list-of-string are
/// read raw from `instance.attributes`.
pub struct AwsEcrRepositoryCreationTemplateConverter {
    service: Arc<EcrService>,
}

impl AwsEcrRepositoryCreationTemplateConverter {
    pub fn new(service: Arc<EcrService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEcrRepositoryCreationTemplateConverter {
    fn resource_type(&self) -> &str {
        "aws_ecr_repository_creation_template"
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

impl AwsEcrRepositoryCreationTemplateConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ecr_gen::RepositoryCreationTemplateTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_ecr_repository_creation_template", e)
            })?;

        let applied_for: Vec<String> = instance
            .attributes
            .get("applied_for")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_default();

        let now = Utc::now().to_rfc3339();
        let template_view = RepositoryCreationTemplateView {
            prefix: model.prefix.clone(),
            description: model.description,
            lifecycle_policy: model.lifecycle_policy,
            repository_policy: model.repository_policy,
            image_tag_mutability: model.image_tag_mutability,
            custom_role_arn: model.custom_role_arn,
            applied_for,
            created_at: Some(now.clone()),
            updated_at: Some(now),
        };

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        // Replace any existing template with the same prefix.
        if let Some(slot) = state_view
            .repository_creation_templates
            .iter_mut()
            .find(|t| t.prefix == model.prefix)
        {
            *slot = template_view;
        } else {
            state_view.repository_creation_templates.push(template_view);
        }
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
        for t in &view.repository_creation_templates {
            let attrs = serde_json::json!({
                "id": t.prefix,
                "prefix": t.prefix,
                "description": t.description,
                "image_tag_mutability": t.image_tag_mutability,
                "lifecycle_policy": t.lifecycle_policy,
                "repository_policy": t.repository_policy,
                "custom_role_arn": t.custom_role_arn,
                "applied_for": t.applied_for,
            });
            results.push(ExtractedResource {
                name: t.prefix.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ecr_repository_policy
// ---------------------------------------------------------------------------

/// Converts `aws_ecr_repository_policy` Terraform resources to/from ECR state.
///
/// State-layer note: like `aws_ecr_lifecycle_policy`, this writes a
/// field on `RepositoryView`. Using `merge` would overwrite the rest
/// of the repository, so snapshot + mutate + restore preserves the
/// other fields.
pub struct AwsEcrRepositoryPolicyConverter {
    service: Arc<EcrService>,
}

impl AwsEcrRepositoryPolicyConverter {
    pub fn new(service: Arc<EcrService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEcrRepositoryPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_ecr_repository_policy"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ecr_repository"]
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

impl AwsEcrRepositoryPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ecr_gen::RepositoryPolicyTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ecr_repository_policy", e))?;

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        match state_view.repositories.get_mut(&model.repository) {
            Some(repo) => {
                repo.repository_policy = Some(model.policy);
            }
            None => {
                warnings.push(format!(
                    "aws_ecr_repository_policy: repository {:?} not found; policy dropped",
                    model.repository
                ));
            }
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
        for repo in view.repositories.values() {
            let Some(policy) = &repo.repository_policy else {
                continue;
            };
            let attrs = serde_json::json!({
                "id": repo.repository_name,
                "repository": repo.repository_name,
                "policy": policy,
                "registry_id": repo.registry_id,
            });
            results.push(ExtractedResource {
                name: repo.repository_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

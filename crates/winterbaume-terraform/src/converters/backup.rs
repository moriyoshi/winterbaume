//! Terraform converters for AWS Backup resources.
//!
//! `VaultTfModel` and `PlanTfModel` are generated from
//! `specs/backup.toml`. The synthesised plan UUID, the `creation_date`
//! constant, the ARN templates, and the raw `rule` /
//! `advanced_backup_setting` nested-block reads are wired up here, plus
//! the `tags_all` raw merge on the plan.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_backup::BackupService;
use winterbaume_backup::views::{BackupPlanView, BackupStateView, BackupVaultView};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::backup as backup_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_backup_vault
// ---------------------------------------------------------------------------

pub struct AwsBackupVaultConverter {
    service: Arc<BackupService>,
}

impl AwsBackupVaultConverter {
    pub fn new(service: Arc<BackupService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBackupVaultConverter {
    fn resource_type(&self) -> &str {
        "aws_backup_vault"
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

impl AwsBackupVaultConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: backup_gen::VaultTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_backup_vault", e))?;

        let name = model.name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:backup:{}:{}:backup-vault:{}",
                region, ctx.default_account_id, name
            )
        });

        let vault_view = BackupVaultView {
            backup_vault_name: name.clone(),
            backup_vault_arn: arn,
            creation_date: Utc::now().to_rfc3339(),
            number_of_recovery_points: 0,
            locked: false,
            min_retention_days: None,
            max_retention_days: None,
            lock_date: None,
            tags: model.tags,
        };

        let mut state_view = minimal_backup_state_view();
        state_view.vaults.insert(name, vault_view);
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
        for vault in view.vaults.values() {
            let attrs = serde_json::json!({
                "id": vault.backup_vault_name,
                "name": vault.backup_vault_name,
                "arn": vault.backup_vault_arn,
                "tags": vault.tags,
                "tags_all": vault.tags,
            });
            results.push(ExtractedResource {
                name: vault.backup_vault_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_backup_plan
// ---------------------------------------------------------------------------

pub struct AwsBackupPlanConverter {
    service: Arc<BackupService>,
}

impl AwsBackupPlanConverter {
    pub fn new(service: Arc<BackupService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBackupPlanConverter {
    fn resource_type(&self) -> &str {
        "aws_backup_plan"
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

impl AwsBackupPlanConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: backup_gen::PlanTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_backup_plan", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let plan_id = model.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:backup:{}:{}:backup-plan:{}",
                region, ctx.default_account_id, plan_id
            )
        });

        // The original converter merges `tags_all` first, then `tags` overrides.
        // `tags_all` is read raw because the spec field is already used for `tags`.
        let mut tags: HashMap<String, String> = HashMap::new();
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags.insert(k.clone(), s.to_string());
                }
            }
        }
        for (k, v) in model.tags {
            tags.insert(k, v);
        }

        // Collect rule blocks as JSON for backup_plan_json
        let rules = attrs
            .get("rule")
            .cloned()
            .unwrap_or(serde_json::Value::Array(vec![]));
        let advanced_backup_settings = attrs
            .get("advanced_backup_setting")
            .cloned()
            .unwrap_or(serde_json::Value::Array(vec![]));
        let backup_plan_json = serde_json::json!({
            "BackupPlanName": name,
            "Rules": rules,
            "AdvancedBackupSettings": advanced_backup_settings,
        });

        let plan_view = BackupPlanView {
            backup_plan_id: plan_id.clone(),
            backup_plan_arn: arn,
            backup_plan_name: name,
            version_id: uuid::Uuid::new_v4().to_string(),
            creation_date: Utc::now().to_rfc3339(),
            backup_plan_json,
            tags,
        };

        let mut state_view = minimal_backup_state_view();
        state_view.backup_plans.insert(plan_id, plan_view);
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
        for plan in view.backup_plans.values() {
            let rule = plan
                .backup_plan_json
                .get("Rules")
                .cloned()
                .unwrap_or(serde_json::Value::Array(vec![]));
            let advanced_backup_setting =
                plan.backup_plan_json.get("AdvancedBackupSettings").cloned();
            let attrs = serde_json::json!({
                "id": plan.backup_plan_id,
                "name": plan.backup_plan_name,
                "arn": plan.backup_plan_arn,
                "version": plan.version_id,
                "tags": plan.tags,
                "rule": rule,
                "advanced_backup_setting": advanced_backup_setting,
            });
            results.push(ExtractedResource {
                name: plan.backup_plan_name.clone(),
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

fn minimal_backup_state_view() -> BackupStateView {
    BackupStateView {
        vaults: HashMap::new(),
        backup_plans: HashMap::new(),
        report_plans: HashMap::new(),
        resource_tags: HashMap::new(),
        ..Default::default()
    }
}

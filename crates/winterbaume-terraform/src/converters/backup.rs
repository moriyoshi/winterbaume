//! Terraform converters for AWS Backup resources.
//!
//! `VaultTfModel` and `PlanTfModel` are generated from
//! `specs/backup.toml`. The synthesised plan UUID, the `creation_date`
//! constant, the ARN templates, and the raw `rule` /
//! `advanced_backup_setting` nested-block reads are wired up here, plus
//! the `tags_all` raw merge on the plan.
//!
//! The 11 newer resources (framework, global_settings,
//! logically_air_gapped_vault, region_settings, report_plan,
//! restore_testing_plan, restore_testing_selection, selection,
//! vault_lock_configuration, vault_notifications, vault_policy) extend
//! the same pattern: scalar fields come from the generated TfModel,
//! while nullable integers, lists, maps, and nested blocks are read
//! raw from `instance.attributes`.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_backup::BackupService;
use winterbaume_backup::views::{
    BackupPlanView, BackupStateView, BackupVaultView, FrameworkView, ReportPlanView,
    RestoreTestingPlanView, RestoreTestingSelectionView, VaultAccessPolicyView,
    VaultNotificationConfigView,
};
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
// aws_backup_framework
// ---------------------------------------------------------------------------

pub struct AwsBackupFrameworkConverter {
    service: Arc<BackupService>,
}

impl AwsBackupFrameworkConverter {
    pub fn new(service: Arc<BackupService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBackupFrameworkConverter {
    fn resource_type(&self) -> &str {
        "aws_backup_framework"
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

impl AwsBackupFrameworkConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: backup_gen::FrameworkTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_backup_framework", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:backup:{}:{}:framework:{}",
                region, ctx.default_account_id, name
            )
        });

        // `control` is a nested-block list; read raw and pass through.
        let framework_controls = attrs
            .get("control")
            .cloned()
            .unwrap_or(serde_json::Value::Array(vec![]));
        let number_of_controls = framework_controls.as_array().map(|a| a.len()).unwrap_or(0) as i32;

        let framework_view = FrameworkView {
            framework_name: name.clone(),
            framework_arn: arn,
            framework_description: model.description.unwrap_or_default(),
            framework_controls,
            creation_time: Utc::now().to_rfc3339(),
            deployment_status: model
                .deployment_status
                .or(model.status)
                .unwrap_or_else(|| "COMPLETED".to_string()),
            number_of_controls,
        };

        let mut state_view = minimal_backup_state_view();
        state_view.frameworks.insert(name, framework_view);
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
        for fw in view.frameworks.values() {
            let attrs = serde_json::json!({
                "id": fw.framework_name,
                "name": fw.framework_name,
                "arn": fw.framework_arn,
                "description": fw.framework_description,
                "deployment_status": fw.deployment_status,
                "status": fw.deployment_status,
                "creation_time": fw.creation_time,
                "control": fw.framework_controls,
                "tags": {},
                "tags_all": {},
            });
            results.push(ExtractedResource {
                name: fw.framework_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_backup_global_settings
// ---------------------------------------------------------------------------
// `BackupStateView` does not surface `global_settings`; the underlying
// `BackupState::global_settings` is reachable only via the typed handler.
// Validate the model so malformed input fails fast, then no-op.

pub struct AwsBackupGlobalSettingsConverter {
    #[allow(dead_code)]
    service: Arc<BackupService>,
}

impl AwsBackupGlobalSettingsConverter {
    pub fn new(service: Arc<BackupService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBackupGlobalSettingsConverter {
    fn resource_type(&self) -> &str {
        "aws_backup_global_settings"
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

impl AwsBackupGlobalSettingsConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: backup_gen::GlobalSettingsTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_backup_global_settings", e))?;
        eprintln!(
            "warning: aws_backup_global_settings: BackupStateView has no slot for global_settings; inject is a no-op"
        );
        Ok(ConversionResult {
            region,
            warnings: vec![
                "aws_backup_global_settings state not persisted (no view slot)".to_string(),
            ],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_backup_logically_air_gapped_vault
// ---------------------------------------------------------------------------
// Stored alongside regular vaults in `BackupStateView.vaults`. The
// min/max retention integers are read raw because the codegen has no
// nullable-integer field type.

pub struct AwsBackupLogicallyAirGappedVaultConverter {
    service: Arc<BackupService>,
}

impl AwsBackupLogicallyAirGappedVaultConverter {
    pub fn new(service: Arc<BackupService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBackupLogicallyAirGappedVaultConverter {
    fn resource_type(&self) -> &str {
        "aws_backup_logically_air_gapped_vault"
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
        // Sub-classified by name in the vaults map; the regular vault
        // converter is the canonical extractor.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsBackupLogicallyAirGappedVaultConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: backup_gen::LogicallyAirGappedVaultTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_backup_logically_air_gapped_vault", e)
            })?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:backup:{}:{}:backup-vault:{}",
                region, ctx.default_account_id, name
            )
        });

        let min_retention_days = attrs.get("min_retention_days").and_then(|v| v.as_i64());
        let max_retention_days = attrs.get("max_retention_days").and_then(|v| v.as_i64());

        let vault_view = BackupVaultView {
            backup_vault_name: name.clone(),
            backup_vault_arn: arn,
            creation_date: Utc::now().to_rfc3339(),
            number_of_recovery_points: 0,
            locked: true,
            min_retention_days,
            max_retention_days,
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
}

// ---------------------------------------------------------------------------
// aws_backup_region_settings
// ---------------------------------------------------------------------------
// `BackupStateView` does not surface `region_settings`. Warning-only.

pub struct AwsBackupRegionSettingsConverter {
    #[allow(dead_code)]
    service: Arc<BackupService>,
}

impl AwsBackupRegionSettingsConverter {
    pub fn new(service: Arc<BackupService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBackupRegionSettingsConverter {
    fn resource_type(&self) -> &str {
        "aws_backup_region_settings"
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

impl AwsBackupRegionSettingsConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: backup_gen::RegionSettingsTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_backup_region_settings", e))?;
        eprintln!(
            "warning: aws_backup_region_settings: BackupStateView has no slot for region_settings; inject is a no-op"
        );
        Ok(ConversionResult {
            region,
            warnings: vec![
                "aws_backup_region_settings state not persisted (no view slot)".to_string(),
            ],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_backup_report_plan
// ---------------------------------------------------------------------------

pub struct AwsBackupReportPlanConverter {
    service: Arc<BackupService>,
}

impl AwsBackupReportPlanConverter {
    pub fn new(service: Arc<BackupService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBackupReportPlanConverter {
    fn resource_type(&self) -> &str {
        "aws_backup_report_plan"
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

impl AwsBackupReportPlanConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: backup_gen::ReportPlanTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_backup_report_plan", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:backup:{}:{}:report-plan:{}",
                region, ctx.default_account_id, name
            )
        });

        // Nested blocks: read raw.
        let report_delivery_channel = attrs
            .get("report_delivery_channel")
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        let report_setting = attrs
            .get("report_setting")
            .cloned()
            .unwrap_or(serde_json::Value::Null);

        let report_plan_view = ReportPlanView {
            report_plan_name: name.clone(),
            report_plan_arn: arn,
            report_plan_description: model.description.unwrap_or_default(),
            report_delivery_channel,
            report_setting,
            creation_time: Utc::now().to_rfc3339(),
            deployment_status: model
                .deployment_status
                .unwrap_or_else(|| "COMPLETED".to_string()),
            tags: model.tags,
        };

        let mut state_view = minimal_backup_state_view();
        state_view.report_plans.insert(name, report_plan_view);
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
        for rp in view.report_plans.values() {
            let attrs = serde_json::json!({
                "id": rp.report_plan_name,
                "name": rp.report_plan_name,
                "arn": rp.report_plan_arn,
                "description": rp.report_plan_description,
                "deployment_status": rp.deployment_status,
                "creation_time": rp.creation_time,
                "report_delivery_channel": rp.report_delivery_channel,
                "report_setting": rp.report_setting,
                "tags": rp.tags,
                "tags_all": rp.tags,
            });
            results.push(ExtractedResource {
                name: rp.report_plan_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_backup_restore_testing_plan
// ---------------------------------------------------------------------------

pub struct AwsBackupRestoreTestingPlanConverter {
    service: Arc<BackupService>,
}

impl AwsBackupRestoreTestingPlanConverter {
    pub fn new(service: Arc<BackupService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBackupRestoreTestingPlanConverter {
    fn resource_type(&self) -> &str {
        "aws_backup_restore_testing_plan"
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

impl AwsBackupRestoreTestingPlanConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: backup_gen::RestoreTestingPlanTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_backup_restore_testing_plan", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:backup:{}:{}:restore-testing-plan:{}",
                region, ctx.default_account_id, name
            )
        });

        let recovery_point_selection = attrs
            .get("recovery_point_selection")
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        let start_window_hours = attrs
            .get("start_window_hours")
            .and_then(|v| v.as_i64())
            .map(|n| n as i32);

        let now = Utc::now().to_rfc3339();
        let plan_view = RestoreTestingPlanView {
            restore_testing_plan_name: name.clone(),
            restore_testing_plan_arn: arn,
            schedule_expression: model.schedule_expression,
            schedule_expression_timezone: model.schedule_expression_timezone,
            start_window_hours,
            recovery_point_selection,
            creator_request_id: None,
            creation_time: now.clone(),
            last_update_time: now,
            tags: model.tags,
        };

        let mut state_view = minimal_backup_state_view();
        state_view.restore_testing_plans.insert(name, plan_view);
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
        for plan in view.restore_testing_plans.values() {
            let attrs = serde_json::json!({
                "id": plan.restore_testing_plan_name,
                "name": plan.restore_testing_plan_name,
                "arn": plan.restore_testing_plan_arn,
                "schedule_expression": plan.schedule_expression,
                "schedule_expression_timezone": plan.schedule_expression_timezone,
                "start_window_hours": plan.start_window_hours,
                "recovery_point_selection": plan.recovery_point_selection,
                "tags": plan.tags,
                "tags_all": plan.tags,
            });
            results.push(ExtractedResource {
                name: plan.restore_testing_plan_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_backup_restore_testing_selection
// ---------------------------------------------------------------------------

pub struct AwsBackupRestoreTestingSelectionConverter {
    service: Arc<BackupService>,
}

impl AwsBackupRestoreTestingSelectionConverter {
    pub fn new(service: Arc<BackupService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBackupRestoreTestingSelectionConverter {
    fn resource_type(&self) -> &str {
        "aws_backup_restore_testing_selection"
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

impl AwsBackupRestoreTestingSelectionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: backup_gen::RestoreTestingSelectionTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_backup_restore_testing_selection", e)
            })?;

        let attrs = &instance.attributes;
        let plan_name = model.restore_testing_plan_name.clone();
        let selection_name = model.name.clone();
        let plan_arn = format!(
            "arn:aws:backup:{}:{}:restore-testing-plan:{}",
            region, ctx.default_account_id, plan_name
        );

        // Optional list/map fields: read raw.
        let protected_resource_arns = attrs
            .get("protected_resource_arns")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let protected_resource_conditions = attrs
            .get("protected_resource_conditions")
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        let restore_metadata_overrides: HashMap<String, String> = attrs
            .get("restore_metadata_overrides")
            .and_then(|v| v.as_object())
            .map(|o| {
                o.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();
        let validation_window_hours = attrs
            .get("validation_window_hours")
            .and_then(|v| v.as_i64())
            .map(|n| n as i32);

        let now = Utc::now().to_rfc3339();
        let selection_view = RestoreTestingSelectionView {
            restore_testing_selection_name: selection_name.clone(),
            restore_testing_plan_name: plan_name.clone(),
            restore_testing_plan_arn: plan_arn,
            iam_role_arn: model.iam_role_arn,
            protected_resource_type: model.protected_resource_type,
            protected_resource_arns,
            protected_resource_conditions,
            restore_metadata_overrides,
            validation_window_hours,
            creator_request_id: None,
            creation_time: now.clone(),
            last_update_time: now,
        };

        let key = format!("{}/{}", plan_name, selection_name);
        let mut state_view = minimal_backup_state_view();
        state_view
            .restore_testing_selections
            .insert(key, selection_view);
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
        for sel in view.restore_testing_selections.values() {
            let attrs = serde_json::json!({
                "id": format!(
                    "{}:{}",
                    sel.restore_testing_plan_name, sel.restore_testing_selection_name
                ),
                "name": sel.restore_testing_selection_name,
                "restore_testing_plan_name": sel.restore_testing_plan_name,
                "iam_role_arn": sel.iam_role_arn,
                "protected_resource_type": sel.protected_resource_type,
                "protected_resource_arns": sel.protected_resource_arns,
                "protected_resource_conditions": sel.protected_resource_conditions,
                "restore_metadata_overrides": sel.restore_metadata_overrides,
                "validation_window_hours": sel.validation_window_hours,
            });
            results.push(ExtractedResource {
                name: sel.restore_testing_selection_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_backup_selection
// ---------------------------------------------------------------------------
// `BackupStateView` does not surface `backup_selections`. Warning-only.

pub struct AwsBackupSelectionConverter {
    #[allow(dead_code)]
    service: Arc<BackupService>,
}

impl AwsBackupSelectionConverter {
    pub fn new(service: Arc<BackupService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBackupSelectionConverter {
    fn resource_type(&self) -> &str {
        "aws_backup_selection"
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

impl AwsBackupSelectionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: backup_gen::SelectionTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_backup_selection", e))?;
        eprintln!(
            "warning: aws_backup_selection: BackupStateView has no slot for backup_selections; inject is a no-op"
        );
        Ok(ConversionResult {
            region,
            warnings: vec!["aws_backup_selection state not persisted (no view slot)".to_string()],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_backup_vault_lock_configuration
// ---------------------------------------------------------------------------
// Modifier on an existing vault: snapshot, set lock fields on the
// matching `BackupVaultView`, and merge back.

pub struct AwsBackupVaultLockConfigurationConverter {
    service: Arc<BackupService>,
}

impl AwsBackupVaultLockConfigurationConverter {
    pub fn new(service: Arc<BackupService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBackupVaultLockConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_backup_vault_lock_configuration"
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
        // Modifier on the vault; not a separately enumerable resource.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsBackupVaultLockConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: backup_gen::VaultLockConfigurationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_backup_vault_lock_configuration", e)
            })?;

        let attrs = &instance.attributes;
        let min_retention_days = attrs.get("min_retention_days").and_then(|v| v.as_i64());
        let max_retention_days = attrs.get("max_retention_days").and_then(|v| v.as_i64());

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        match snapshot.vaults.get_mut(&model.backup_vault_name) {
            Some(vault) => {
                vault.locked = true;
                if min_retention_days.is_some() {
                    vault.min_retention_days = min_retention_days;
                }
                if max_retention_days.is_some() {
                    vault.max_retention_days = max_retention_days;
                }
                vault.lock_date = Some(Utc::now().to_rfc3339());
                if let Some(arn) = model.backup_vault_arn {
                    vault.backup_vault_arn = arn;
                }
            }
            None => {
                eprintln!(
                    "warning: aws_backup_vault_lock_configuration: vault {} not found; lock dropped",
                    model.backup_vault_name
                );
                warnings.push(format!(
                    "aws_backup_vault_lock_configuration: vault {} missing",
                    model.backup_vault_name
                ));
            }
        }
        self.service
            .merge(&ctx.default_account_id, &region, snapshot)
            .await?;
        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_backup_vault_notifications
// ---------------------------------------------------------------------------

pub struct AwsBackupVaultNotificationsConverter {
    service: Arc<BackupService>,
}

impl AwsBackupVaultNotificationsConverter {
    pub fn new(service: Arc<BackupService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBackupVaultNotificationsConverter {
    fn resource_type(&self) -> &str {
        "aws_backup_vault_notifications"
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

impl AwsBackupVaultNotificationsConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: backup_gen::VaultNotificationsTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_backup_vault_notifications", e))?;

        let attrs = &instance.attributes;
        let backup_vault_arn = model.backup_vault_arn.unwrap_or_else(|| {
            format!(
                "arn:aws:backup:{}:{}:backup-vault:{}",
                region, ctx.default_account_id, model.backup_vault_name
            )
        });
        let backup_vault_events = attrs
            .get("backup_vault_events")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let notif_view = VaultNotificationConfigView {
            backup_vault_name: model.backup_vault_name.clone(),
            backup_vault_arn,
            sns_topic_arn: model.sns_topic_arn,
            backup_vault_events,
        };

        let mut state_view = minimal_backup_state_view();
        state_view
            .vault_notifications
            .insert(model.backup_vault_name, notif_view);
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
        for notif in view.vault_notifications.values() {
            let attrs = serde_json::json!({
                "id": notif.backup_vault_name,
                "backup_vault_name": notif.backup_vault_name,
                "backup_vault_arn": notif.backup_vault_arn,
                "sns_topic_arn": notif.sns_topic_arn,
                "backup_vault_events": notif.backup_vault_events,
            });
            results.push(ExtractedResource {
                name: notif.backup_vault_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_backup_vault_policy
// ---------------------------------------------------------------------------

pub struct AwsBackupVaultPolicyConverter {
    service: Arc<BackupService>,
}

impl AwsBackupVaultPolicyConverter {
    pub fn new(service: Arc<BackupService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBackupVaultPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_backup_vault_policy"
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

impl AwsBackupVaultPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: backup_gen::VaultPolicyTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_backup_vault_policy", e))?;

        let backup_vault_arn = model.backup_vault_arn.unwrap_or_else(|| {
            format!(
                "arn:aws:backup:{}:{}:backup-vault:{}",
                region, ctx.default_account_id, model.backup_vault_name
            )
        });

        let policy_view = VaultAccessPolicyView {
            backup_vault_name: model.backup_vault_name.clone(),
            backup_vault_arn,
            policy: model.policy,
        };

        let mut state_view = minimal_backup_state_view();
        state_view
            .vault_access_policies
            .insert(model.backup_vault_name, policy_view);
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
        for policy in view.vault_access_policies.values() {
            let attrs = serde_json::json!({
                "id": policy.backup_vault_name,
                "backup_vault_name": policy.backup_vault_name,
                "backup_vault_arn": policy.backup_vault_arn,
                "policy": policy.policy,
            });
            results.push(ExtractedResource {
                name: policy.backup_vault_name.clone(),
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

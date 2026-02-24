use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;
use uuid::Uuid;

use crate::types::*;

#[derive(Debug, Default)]
pub struct BackupState {
    pub vaults: HashMap<String, BackupVault>,
    pub backup_plans: HashMap<String, BackupPlanData>,
    /// Selections keyed by selection_id.
    pub backup_selections: HashMap<String, BackupSelectionData>,
    pub report_plans: HashMap<String, ReportPlanData>,
    /// Recovery points keyed by ARN.
    pub recovery_points: HashMap<String, RecoveryPointData>,
    /// Backup jobs keyed by job_id.
    pub backup_jobs: HashMap<String, BackupJobData>,
    /// Tags keyed by resource ARN.
    pub resource_tags: HashMap<String, HashMap<String, String>>,
    /// Vault access policies keyed by vault name.
    pub vault_access_policies: HashMap<String, VaultAccessPolicy>,
    /// Vault notification configurations keyed by vault name.
    pub vault_notifications: HashMap<String, VaultNotificationConfig>,
    /// Audit frameworks keyed by framework name.
    pub frameworks: HashMap<String, FrameworkData>,
    /// Global settings (account-level).
    pub global_settings: GlobalSettings,
    /// Region settings.
    pub region_settings: RegionSettings,
    /// Report jobs keyed by job_id.
    pub report_jobs: HashMap<String, ReportJobData>,
    /// Scan jobs keyed by scan_job_id.
    pub scan_jobs: HashMap<String, ScanJobData>,
    /// Tiering configurations keyed by configuration name.
    pub tiering_configs: HashMap<String, TieringConfigData>,
    /// Legal holds keyed by legal_hold_id.
    pub legal_holds: HashMap<String, LegalHoldData>,
    /// Copy jobs keyed by copy_job_id.
    pub copy_jobs: HashMap<String, CopyJobData>,
    /// Restore jobs keyed by restore_job_id.
    pub restore_jobs: HashMap<String, RestoreJobData>,
    /// Restore testing plans keyed by plan name.
    pub restore_testing_plans: HashMap<String, RestoreTestingPlanData>,
    /// Restore testing selections keyed by (plan_name, selection_name).
    pub restore_testing_selections: HashMap<(String, String), RestoreTestingSelectionData>,
}

#[derive(Debug, Error)]
pub enum BackupError {
    #[error("Backup vault {0} already exists.")]
    VaultAlreadyExists(String),

    #[error("Backup vault {0} does not exist.")]
    VaultNotFound(String),

    #[error("Backup plan with name {0} already exists.")]
    PlanAlreadyExists(String),

    #[error("Backup plan {0} does not exist.")]
    PlanNotFound(String),

    #[error("Report plan {0} already exists.")]
    ReportPlanAlreadyExists(String),

    #[error("Report plan {0} does not exist.")]
    ReportPlanNotFound(String),

    #[error("Framework {0} already exists.")]
    FrameworkAlreadyExists(String),

    #[error("Framework {0} does not exist.")]
    FrameworkNotFound(String),

    #[error("Backup selection {0} does not exist.")]
    SelectionNotFound(String),

    #[error("Recovery point {0} does not exist.")]
    RecoveryPointNotFound(String),

    #[error("Backup vault access policy for {0} does not exist.")]
    VaultAccessPolicyNotFound(String),

    #[error("Notification configuration for vault {0} does not exist.")]
    VaultNotificationsNotFound(String),

    #[error("Backup job {0} does not exist.")]
    BackupJobNotFound(String),

    #[error("Report job {0} does not exist.")]
    ReportJobNotFound(String),

    #[error("Scan job {0} does not exist.")]
    ScanJobNotFound(String),

    #[error("Tiering configuration {0} already exists.")]
    TieringConfigAlreadyExists(String),

    #[error("Tiering configuration {0} does not exist.")]
    TieringConfigNotFound(String),

    #[error("Vault {0} does not have a lock configured.")]
    VaultNotLocked(String),

    #[error("Backup job {0} is not in a cancellable state.")]
    BackupJobNotCancellable(String),

    #[error("Legal hold {0} already exists.")]
    LegalHoldAlreadyExists(String),

    #[error("Legal hold {0} does not exist.")]
    LegalHoldNotFound(String),

    #[error("Copy job {0} does not exist.")]
    CopyJobNotFound(String),

    #[error("Restore job {0} does not exist.")]
    RestoreJobNotFound(String),

    #[error("Restore testing plan {0} already exists.")]
    RestoreTestingPlanAlreadyExists(String),

    #[error("Restore testing plan {0} does not exist.")]
    RestoreTestingPlanNotFound(String),

    #[error("Restore testing selection {0} already exists.")]
    RestoreTestingSelectionAlreadyExists(String),

    #[error("Restore testing selection {0} does not exist.")]
    RestoreTestingSelectionNotFound(String),
}

impl BackupState {
    pub fn create_backup_vault(
        &mut self,
        name: &str,
        arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<&BackupVault, BackupError> {
        if self.vaults.contains_key(name) {
            return Err(BackupError::VaultAlreadyExists(name.to_string()));
        }

        if !tags.is_empty() {
            self.resource_tags.insert(arn.to_string(), tags.clone());
        }

        let vault = BackupVault {
            backup_vault_name: name.to_string(),
            backup_vault_arn: arn.to_string(),
            creation_date: Utc::now(),
            number_of_recovery_points: 0,
            locked: false,
            min_retention_days: None,
            max_retention_days: None,
            lock_date: None,
            tags,
        };

        self.vaults.insert(name.to_string(), vault);
        Ok(self.vaults.get(name).unwrap())
    }

    pub fn describe_backup_vault(&self, name: &str) -> Result<&BackupVault, BackupError> {
        self.vaults
            .get(name)
            .ok_or_else(|| BackupError::VaultNotFound(name.to_string()))
    }

    pub fn delete_backup_vault(&mut self, name: &str) -> Result<(), BackupError> {
        if self.vaults.remove(name).is_none() {
            return Err(BackupError::VaultNotFound(name.to_string()));
        }
        Ok(())
    }

    pub fn list_backup_vaults(&self) -> Vec<&BackupVault> {
        self.vaults.values().collect()
    }

    // --- Backup Plan operations ---

    pub fn create_backup_plan(
        &mut self,
        name: &str,
        plan_json: &serde_json::Value,
        region: &str,
        account_id: &str,
        tags: HashMap<String, String>,
    ) -> Result<&BackupPlanData, BackupError> {
        // Check for duplicate plan names
        let already_exists = self
            .backup_plans
            .values()
            .any(|p| p.backup_plan_name == name);
        if already_exists {
            return Err(BackupError::PlanAlreadyExists(name.to_string()));
        }

        let plan_id = Uuid::new_v4().to_string();
        let version_id = Uuid::new_v4().to_string();
        let arn = format!("arn:aws:backup:{region}:{account_id}:backup-plan:{plan_id}");

        // Generate rule IDs for each rule in the plan JSON
        let enriched_json =
            if let Some(rules_arr) = plan_json.get("Rules").and_then(|v| v.as_array()) {
                let enriched_rules: Vec<serde_json::Value> = rules_arr
                    .iter()
                    .map(|rule| {
                        let mut r = rule.clone();
                        if let serde_json::Value::Object(ref mut map) = r {
                            map.entry("RuleId").or_insert_with(|| {
                                serde_json::Value::String(Uuid::new_v4().to_string())
                            });
                        }
                        r
                    })
                    .collect();
                let mut enriched = plan_json.clone();
                if let serde_json::Value::Object(ref mut map) = enriched {
                    map.insert(
                        "Rules".to_string(),
                        serde_json::Value::Array(enriched_rules),
                    );
                }
                enriched
            } else {
                plan_json.clone()
            };

        let plan = BackupPlanData {
            backup_plan_id: plan_id.clone(),
            backup_plan_arn: arn.clone(),
            backup_plan_name: name.to_string(),
            version_id,
            creation_date: Utc::now(),
            backup_plan_json: enriched_json,
            tags: tags.clone(),
        };

        if !tags.is_empty() {
            self.resource_tags.insert(arn, tags);
        }

        self.backup_plans.insert(plan_id.clone(), plan);
        Ok(self.backup_plans.get(&plan_id).unwrap())
    }

    pub fn get_backup_plan(&self, plan_id: &str) -> Result<&BackupPlanData, BackupError> {
        self.backup_plans
            .get(plan_id)
            .ok_or_else(|| BackupError::PlanNotFound(plan_id.to_string()))
    }

    pub fn delete_backup_plan(&mut self, plan_id: &str) -> Result<BackupPlanData, BackupError> {
        self.backup_plans
            .remove(plan_id)
            .ok_or_else(|| BackupError::PlanNotFound(plan_id.to_string()))
    }

    pub fn list_backup_plans(&self) -> Vec<&BackupPlanData> {
        self.backup_plans.values().collect()
    }

    // --- Report Plan operations ---

    pub fn create_report_plan(
        &mut self,
        name: &str,
        description: &str,
        delivery_channel: &serde_json::Value,
        report_setting: &serde_json::Value,
        region: &str,
        account_id: &str,
        tags: HashMap<String, String>,
    ) -> Result<&ReportPlanData, BackupError> {
        if self.report_plans.contains_key(name) {
            return Err(BackupError::ReportPlanAlreadyExists(name.to_string()));
        }

        let arn = format!("arn:aws:backup:{region}:{account_id}:report-plan:{name}");

        let plan = ReportPlanData {
            report_plan_name: name.to_string(),
            report_plan_arn: arn.clone(),
            report_plan_description: description.to_string(),
            report_delivery_channel: delivery_channel.clone(),
            report_setting: report_setting.clone(),
            creation_time: Utc::now(),
            deployment_status: "COMPLETED".to_string(),
            tags: tags.clone(),
        };

        if !tags.is_empty() {
            self.resource_tags.insert(arn, tags);
        }

        self.report_plans.insert(name.to_string(), plan);
        Ok(self.report_plans.get(name).unwrap())
    }

    pub fn describe_report_plan(&self, name: &str) -> Result<&ReportPlanData, BackupError> {
        self.report_plans
            .get(name)
            .ok_or_else(|| BackupError::ReportPlanNotFound(name.to_string()))
    }

    pub fn delete_report_plan(&mut self, name: &str) -> Result<(), BackupError> {
        if self.report_plans.remove(name).is_none() {
            return Err(BackupError::ReportPlanNotFound(name.to_string()));
        }
        Ok(())
    }

    pub fn list_report_plans(&self) -> Vec<&ReportPlanData> {
        self.report_plans.values().collect()
    }

    // --- Vault lock operations ---

    pub fn put_backup_vault_lock_configuration(
        &mut self,
        vault_name: &str,
        min_retention_days: Option<i64>,
        max_retention_days: Option<i64>,
    ) -> Result<(), BackupError> {
        let vault = self
            .vaults
            .get_mut(vault_name)
            .ok_or_else(|| BackupError::VaultNotFound(vault_name.to_string()))?;
        vault.locked = true;
        vault.min_retention_days = min_retention_days;
        vault.max_retention_days = max_retention_days;
        vault.lock_date = Some(Utc::now());
        Ok(())
    }

    pub fn delete_backup_vault_lock_configuration(
        &mut self,
        vault_name: &str,
    ) -> Result<(), BackupError> {
        let vault = self
            .vaults
            .get_mut(vault_name)
            .ok_or_else(|| BackupError::VaultNotFound(vault_name.to_string()))?;
        if !vault.locked {
            return Err(BackupError::VaultNotLocked(vault_name.to_string()));
        }
        vault.locked = false;
        vault.min_retention_days = None;
        vault.max_retention_days = None;
        vault.lock_date = None;
        Ok(())
    }

    // --- Tag operations ---

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), BackupError> {
        let entry = self
            .resource_tags
            .entry(resource_arn.to_string())
            .or_default();
        entry.extend(tags);
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), BackupError> {
        if let Some(tags) = self.resource_tags.get_mut(resource_arn) {
            for key in tag_keys {
                tags.remove(key);
            }
        }
        Ok(())
    }

    pub fn list_tags(&self, resource_arn: &str) -> HashMap<String, String> {
        self.resource_tags
            .get(resource_arn)
            .cloned()
            .unwrap_or_default()
    }

    // --- Backup Selection operations ---

    pub fn create_backup_selection(
        &mut self,
        plan_id: &str,
        selection_name: &str,
        iam_role_arn: &str,
        resources: Vec<String>,
        selection_json: serde_json::Value,
    ) -> Result<&BackupSelectionData, BackupError> {
        if !self.backup_plans.contains_key(plan_id) {
            return Err(BackupError::PlanNotFound(plan_id.to_string()));
        }

        let selection_id = Uuid::new_v4().to_string();
        let selection = BackupSelectionData {
            selection_id: selection_id.clone(),
            backup_plan_id: plan_id.to_string(),
            selection_name: selection_name.to_string(),
            iam_role_arn: iam_role_arn.to_string(),
            resources,
            creation_date: Utc::now(),
            selection_json,
        };

        self.backup_selections
            .insert(selection_id.clone(), selection);
        Ok(self.backup_selections.get(&selection_id).unwrap())
    }

    pub fn get_backup_selection(
        &self,
        plan_id: &str,
        selection_id: &str,
    ) -> Result<&BackupSelectionData, BackupError> {
        self.backup_selections
            .get(selection_id)
            .filter(|s| s.backup_plan_id == plan_id)
            .ok_or_else(|| BackupError::SelectionNotFound(selection_id.to_string()))
    }

    pub fn delete_backup_selection(
        &mut self,
        plan_id: &str,
        selection_id: &str,
    ) -> Result<(), BackupError> {
        let exists = self
            .backup_selections
            .get(selection_id)
            .map(|s| s.backup_plan_id == plan_id)
            .unwrap_or(false);
        if !exists {
            return Err(BackupError::SelectionNotFound(selection_id.to_string()));
        }
        self.backup_selections.remove(selection_id);
        Ok(())
    }

    pub fn list_backup_selections(&self, plan_id: &str) -> Vec<&BackupSelectionData> {
        self.backup_selections
            .values()
            .filter(|s| s.backup_plan_id == plan_id)
            .collect()
    }

    // --- Recovery Point operations ---

    pub fn create_recovery_point(
        &mut self,
        vault_name: &str,
        vault_arn: &str,
        resource_arn: &str,
        resource_type: &str,
        iam_role_arn: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&RecoveryPointData, BackupError> {
        if !self.vaults.contains_key(vault_name) {
            return Err(BackupError::VaultNotFound(vault_name.to_string()));
        }

        let recovery_point_id = Uuid::new_v4().to_string();
        let recovery_point_arn =
            format!("arn:aws:backup:{region}:{account_id}:recovery-point:{recovery_point_id}");

        let rp = RecoveryPointData {
            recovery_point_arn: recovery_point_arn.clone(),
            backup_vault_name: vault_name.to_string(),
            backup_vault_arn: vault_arn.to_string(),
            resource_arn: resource_arn.to_string(),
            resource_type: resource_type.to_string(),
            iam_role_arn: iam_role_arn.to_string(),
            status: "COMPLETED".to_string(),
            creation_date: Utc::now(),
            backup_size_bytes: 0,
            account_id: account_id.to_string(),
        };

        // Update vault recovery point count
        if let Some(vault) = self.vaults.get_mut(vault_name) {
            vault.number_of_recovery_points += 1;
        }

        self.recovery_points.insert(recovery_point_arn.clone(), rp);
        Ok(self.recovery_points.get(&recovery_point_arn).unwrap())
    }

    pub fn describe_recovery_point(
        &self,
        vault_name: &str,
        recovery_point_arn: &str,
    ) -> Result<&RecoveryPointData, BackupError> {
        self.recovery_points
            .get(recovery_point_arn)
            .filter(|rp| rp.backup_vault_name == vault_name)
            .ok_or_else(|| BackupError::RecoveryPointNotFound(recovery_point_arn.to_string()))
    }

    pub fn delete_recovery_point(
        &mut self,
        vault_name: &str,
        recovery_point_arn: &str,
    ) -> Result<(), BackupError> {
        let exists = self
            .recovery_points
            .get(recovery_point_arn)
            .map(|rp| rp.backup_vault_name == vault_name)
            .unwrap_or(false);
        if !exists {
            return Err(BackupError::RecoveryPointNotFound(
                recovery_point_arn.to_string(),
            ));
        }
        self.recovery_points.remove(recovery_point_arn);
        // Update vault recovery point count
        if let Some(vault) = self.vaults.get_mut(vault_name) {
            vault.number_of_recovery_points = vault.number_of_recovery_points.saturating_sub(1);
        }
        Ok(())
    }

    pub fn list_recovery_points_by_backup_vault(
        &self,
        vault_name: &str,
    ) -> Vec<&RecoveryPointData> {
        self.recovery_points
            .values()
            .filter(|rp| rp.backup_vault_name == vault_name)
            .collect()
    }

    // --- Backup Job operations ---

    pub fn start_backup_job(
        &mut self,
        vault_name: &str,
        vault_arn: &str,
        resource_arn: &str,
        resource_type: &str,
        iam_role_arn: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&BackupJobData, BackupError> {
        if !self.vaults.contains_key(vault_name) {
            return Err(BackupError::VaultNotFound(vault_name.to_string()));
        }

        let job_id = Uuid::new_v4().to_string();
        let recovery_point_id = Uuid::new_v4().to_string();
        let recovery_point_arn =
            format!("arn:aws:backup:{region}:{account_id}:recovery-point:{recovery_point_id}");

        let job = BackupJobData {
            backup_job_id: job_id.clone(),
            backup_vault_name: vault_name.to_string(),
            backup_vault_arn: vault_arn.to_string(),
            recovery_point_arn: recovery_point_arn.clone(),
            resource_arn: resource_arn.to_string(),
            resource_type: resource_type.to_string(),
            iam_role_arn: iam_role_arn.to_string(),
            state: "RUNNING".to_string(),
            creation_date: Utc::now(),
            completion_date: None,
            account_id: account_id.to_string(),
        };

        self.backup_jobs.insert(job_id.clone(), job);
        Ok(self.backup_jobs.get(&job_id).unwrap())
    }

    pub fn describe_backup_job(&self, job_id: &str) -> Result<&BackupJobData, BackupError> {
        self.backup_jobs
            .get(job_id)
            .ok_or_else(|| BackupError::BackupJobNotFound(job_id.to_string()))
    }

    pub fn stop_backup_job(&mut self, job_id: &str) -> Result<(), BackupError> {
        let job = self
            .backup_jobs
            .get_mut(job_id)
            .ok_or_else(|| BackupError::BackupJobNotFound(job_id.to_string()))?;
        if job.state == "COMPLETED" || job.state == "ABORTED" {
            return Err(BackupError::BackupJobNotCancellable(job_id.to_string()));
        }
        job.state = "ABORTED".to_string();
        job.completion_date = Some(Utc::now());
        Ok(())
    }

    pub fn list_backup_jobs(&self) -> Vec<&BackupJobData> {
        self.backup_jobs.values().collect()
    }

    // --- Vault Access Policy operations ---

    pub fn put_backup_vault_access_policy(
        &mut self,
        vault_name: &str,
        vault_arn: &str,
        policy: &str,
    ) -> Result<(), BackupError> {
        if !self.vaults.contains_key(vault_name) {
            return Err(BackupError::VaultNotFound(vault_name.to_string()));
        }
        self.vault_access_policies.insert(
            vault_name.to_string(),
            VaultAccessPolicy {
                backup_vault_name: vault_name.to_string(),
                backup_vault_arn: vault_arn.to_string(),
                policy: policy.to_string(),
            },
        );
        Ok(())
    }

    pub fn get_backup_vault_access_policy(
        &self,
        vault_name: &str,
    ) -> Result<&VaultAccessPolicy, BackupError> {
        self.vault_access_policies
            .get(vault_name)
            .ok_or_else(|| BackupError::VaultAccessPolicyNotFound(vault_name.to_string()))
    }

    pub fn delete_backup_vault_access_policy(
        &mut self,
        vault_name: &str,
    ) -> Result<(), BackupError> {
        if !self.vaults.contains_key(vault_name) {
            return Err(BackupError::VaultNotFound(vault_name.to_string()));
        }
        self.vault_access_policies.remove(vault_name);
        Ok(())
    }

    // --- Vault Notification operations ---

    pub fn put_backup_vault_notifications(
        &mut self,
        vault_name: &str,
        vault_arn: &str,
        sns_topic_arn: &str,
        backup_vault_events: Vec<String>,
    ) -> Result<(), BackupError> {
        if !self.vaults.contains_key(vault_name) {
            return Err(BackupError::VaultNotFound(vault_name.to_string()));
        }
        self.vault_notifications.insert(
            vault_name.to_string(),
            VaultNotificationConfig {
                backup_vault_name: vault_name.to_string(),
                backup_vault_arn: vault_arn.to_string(),
                sns_topic_arn: sns_topic_arn.to_string(),
                backup_vault_events,
            },
        );
        Ok(())
    }

    pub fn get_backup_vault_notifications(
        &self,
        vault_name: &str,
    ) -> Result<&VaultNotificationConfig, BackupError> {
        self.vault_notifications
            .get(vault_name)
            .ok_or_else(|| BackupError::VaultNotificationsNotFound(vault_name.to_string()))
    }

    pub fn delete_backup_vault_notifications(
        &mut self,
        vault_name: &str,
    ) -> Result<(), BackupError> {
        if !self.vaults.contains_key(vault_name) {
            return Err(BackupError::VaultNotFound(vault_name.to_string()));
        }
        self.vault_notifications.remove(vault_name);
        Ok(())
    }

    // --- Framework operations ---

    #[allow(clippy::too_many_arguments)]
    pub fn create_framework(
        &mut self,
        name: &str,
        description: &str,
        controls: serde_json::Value,
        region: &str,
        account_id: &str,
        tags: HashMap<String, String>,
    ) -> Result<&FrameworkData, BackupError> {
        if self.frameworks.contains_key(name) {
            return Err(BackupError::FrameworkAlreadyExists(name.to_string()));
        }

        let arn = format!("arn:aws:backup:{region}:{account_id}:framework:{name}");
        let num_controls = controls.as_array().map(|a| a.len() as i32).unwrap_or(0);

        let framework = FrameworkData {
            framework_name: name.to_string(),
            framework_arn: arn.clone(),
            framework_description: description.to_string(),
            framework_controls: controls,
            creation_time: Utc::now(),
            deployment_status: "COMPLETED".to_string(),
            number_of_controls: num_controls,
        };

        if !tags.is_empty() {
            self.resource_tags.insert(arn, tags);
        }

        self.frameworks.insert(name.to_string(), framework);
        Ok(self.frameworks.get(name).unwrap())
    }

    pub fn describe_framework(&self, name: &str) -> Result<&FrameworkData, BackupError> {
        self.frameworks
            .get(name)
            .ok_or_else(|| BackupError::FrameworkNotFound(name.to_string()))
    }

    pub fn delete_framework(&mut self, name: &str) -> Result<(), BackupError> {
        if self.frameworks.remove(name).is_none() {
            return Err(BackupError::FrameworkNotFound(name.to_string()));
        }
        Ok(())
    }

    pub fn update_framework(
        &mut self,
        name: &str,
        description: Option<&str>,
        controls: Option<serde_json::Value>,
    ) -> Result<&FrameworkData, BackupError> {
        let framework = self
            .frameworks
            .get_mut(name)
            .ok_or_else(|| BackupError::FrameworkNotFound(name.to_string()))?;
        if let Some(desc) = description {
            framework.framework_description = desc.to_string();
        }
        if let Some(c) = controls {
            framework.number_of_controls = c.as_array().map(|a| a.len() as i32).unwrap_or(0);
            framework.framework_controls = c;
        }
        Ok(self.frameworks.get(name).unwrap())
    }

    pub fn list_frameworks(&self) -> Vec<&FrameworkData> {
        self.frameworks.values().collect()
    }

    // --- Global Settings operations ---

    pub fn update_global_settings(&mut self, settings: HashMap<String, String>) {
        self.global_settings.global_settings.extend(settings);
    }

    pub fn describe_global_settings(&self) -> &GlobalSettings {
        &self.global_settings
    }

    // --- Region Settings operations ---

    pub fn update_region_settings(
        &mut self,
        opt_in: Option<HashMap<String, bool>>,
        management: Option<HashMap<String, bool>>,
    ) {
        if let Some(p) = opt_in {
            self.region_settings
                .resource_type_opt_in_preference
                .extend(p);
        }
        if let Some(p) = management {
            self.region_settings
                .resource_type_management_preference
                .extend(p);
        }
    }

    pub fn describe_region_settings(&self) -> &RegionSettings {
        &self.region_settings
    }

    // --- Report Job operations ---

    pub fn start_report_job(
        &mut self,
        report_plan_name: &str,
        region: &str,
        account_id: &str,
    ) -> Result<&ReportJobData, BackupError> {
        let plan = self
            .report_plans
            .get(report_plan_name)
            .ok_or_else(|| BackupError::ReportPlanNotFound(report_plan_name.to_string()))?;
        let report_plan_arn = plan.report_plan_arn.clone();
        let report_template = plan
            .report_setting
            .get("ReportTemplate")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let job_id = Uuid::new_v4().to_string();
        let job = ReportJobData {
            report_job_id: job_id.clone(),
            report_plan_arn,
            report_template,
            creation_time: Utc::now(),
            completion_time: Some(Utc::now()),
            status: "COMPLETED".to_string(),
        };

        self.report_jobs.insert(job_id.clone(), job);
        Ok(self.report_jobs.get(&job_id).unwrap())
    }

    pub fn describe_report_job(&self, job_id: &str) -> Result<&ReportJobData, BackupError> {
        self.report_jobs
            .get(job_id)
            .ok_or_else(|| BackupError::ReportJobNotFound(job_id.to_string()))
    }

    pub fn list_report_jobs(&self, report_plan_name: Option<&str>) -> Vec<&ReportJobData> {
        self.report_jobs
            .values()
            .filter(|j| {
                if let Some(name) = report_plan_name {
                    j.report_plan_arn.ends_with(&format!(":{name}"))
                } else {
                    true
                }
            })
            .collect()
    }

    // --- Scan Job operations ---

    pub fn start_scan_job(
        &mut self,
        vault_name: &str,
        vault_arn: &str,
        recovery_point_arn: &str,
        iam_role_arn: &str,
        malware_scanner: &str,
        scan_mode: &str,
        scanner_role_arn: &str,
        scan_base_recovery_point_arn: Option<String>,
        account_id: &str,
        region: &str,
    ) -> Result<&ScanJobData, BackupError> {
        let scan_job_id = Uuid::new_v4().to_string();
        let job = ScanJobData {
            scan_job_id: scan_job_id.clone(),
            backup_vault_name: vault_name.to_string(),
            backup_vault_arn: vault_arn.to_string(),
            recovery_point_arn: recovery_point_arn.to_string(),
            iam_role_arn: iam_role_arn.to_string(),
            malware_scanner: malware_scanner.to_string(),
            scan_mode: scan_mode.to_string(),
            scanner_role_arn: scanner_role_arn.to_string(),
            scan_base_recovery_point_arn,
            state: "RUNNING".to_string(),
            creation_date: Utc::now(),
            completion_date: None,
            account_id: account_id.to_string(),
        };
        self.scan_jobs.insert(scan_job_id.clone(), job);
        Ok(self.scan_jobs.get(&scan_job_id).unwrap())
    }

    pub fn describe_scan_job(&self, scan_job_id: &str) -> Result<&ScanJobData, BackupError> {
        self.scan_jobs
            .get(scan_job_id)
            .ok_or_else(|| BackupError::ScanJobNotFound(scan_job_id.to_string()))
    }

    pub fn list_scan_jobs(&self) -> Vec<&ScanJobData> {
        self.scan_jobs.values().collect()
    }

    // --- Tiering Configuration operations ---

    pub fn create_tiering_configuration(
        &mut self,
        name: &str,
        vault_name: &str,
        resource_selection: serde_json::Value,
        creator_request_id: Option<String>,
        region: &str,
        account_id: &str,
        tags: HashMap<String, String>,
    ) -> Result<&TieringConfigData, BackupError> {
        if self.tiering_configs.contains_key(name) {
            return Err(BackupError::TieringConfigAlreadyExists(name.to_string()));
        }

        let arn = format!("arn:aws:backup:{region}:{account_id}:tiering-configuration:{name}");
        let now = Utc::now();

        let config = TieringConfigData {
            tiering_configuration_name: name.to_string(),
            tiering_configuration_arn: arn.clone(),
            backup_vault_name: vault_name.to_string(),
            resource_selection,
            creation_time: now,
            last_updated_time: now,
            creator_request_id,
            tags: tags.clone(),
        };

        if !tags.is_empty() {
            self.resource_tags.insert(arn, tags);
        }

        self.tiering_configs.insert(name.to_string(), config);
        Ok(self.tiering_configs.get(name).unwrap())
    }

    pub fn get_tiering_configuration(&self, name: &str) -> Result<&TieringConfigData, BackupError> {
        self.tiering_configs
            .get(name)
            .ok_or_else(|| BackupError::TieringConfigNotFound(name.to_string()))
    }

    pub fn delete_tiering_configuration(&mut self, name: &str) -> Result<(), BackupError> {
        if self.tiering_configs.remove(name).is_none() {
            return Err(BackupError::TieringConfigNotFound(name.to_string()));
        }
        Ok(())
    }

    pub fn list_tiering_configurations(&self) -> Vec<&TieringConfigData> {
        self.tiering_configs.values().collect()
    }

    pub fn update_tiering_configuration(
        &mut self,
        name: &str,
        vault_name: Option<&str>,
        resource_selection: Option<serde_json::Value>,
    ) -> Result<&TieringConfigData, BackupError> {
        let config = self
            .tiering_configs
            .get_mut(name)
            .ok_or_else(|| BackupError::TieringConfigNotFound(name.to_string()))?;
        if let Some(vn) = vault_name {
            config.backup_vault_name = vn.to_string();
        }
        if let Some(rs) = resource_selection {
            config.resource_selection = rs;
        }
        config.last_updated_time = Utc::now();
        Ok(self.tiering_configs.get(name).unwrap())
    }

    // --- UpdateReportPlan ---

    pub fn update_report_plan(
        &mut self,
        name: &str,
        description: Option<&str>,
        delivery_channel: Option<serde_json::Value>,
        report_setting: Option<serde_json::Value>,
    ) -> Result<&ReportPlanData, BackupError> {
        let plan = self
            .report_plans
            .get_mut(name)
            .ok_or_else(|| BackupError::ReportPlanNotFound(name.to_string()))?;
        if let Some(desc) = description {
            plan.report_plan_description = desc.to_string();
        }
        if let Some(dc) = delivery_channel {
            plan.report_delivery_channel = dc;
        }
        if let Some(rs) = report_setting {
            plan.report_setting = rs;
        }
        Ok(self.report_plans.get(name).unwrap())
    }

    // --- Legal Hold operations ---

    pub fn create_legal_hold(
        &mut self,
        title: &str,
        description: &str,
        recovery_point_selection: serde_json::Value,
        region: &str,
        account_id: &str,
        tags: HashMap<String, String>,
    ) -> Result<&LegalHoldData, BackupError> {
        let legal_hold_id = Uuid::new_v4().to_string();
        let arn = format!("arn:aws:backup:{region}:{account_id}:legal-hold:{legal_hold_id}");

        let hold = LegalHoldData {
            legal_hold_id: legal_hold_id.clone(),
            legal_hold_arn: arn.clone(),
            title: title.to_string(),
            description: description.to_string(),
            status: "ACTIVE".to_string(),
            creation_date: Utc::now(),
            cancellation_date: None,
            recovery_point_selection,
            tags: tags.clone(),
        };

        if !tags.is_empty() {
            self.resource_tags.insert(arn, tags);
        }

        self.legal_holds.insert(legal_hold_id.clone(), hold);
        Ok(self.legal_holds.get(&legal_hold_id).unwrap())
    }

    pub fn cancel_legal_hold(&mut self, legal_hold_id: &str) -> Result<(), BackupError> {
        let hold = self
            .legal_holds
            .get_mut(legal_hold_id)
            .ok_or_else(|| BackupError::LegalHoldNotFound(legal_hold_id.to_string()))?;
        hold.status = "CANCELED".to_string();
        hold.cancellation_date = Some(Utc::now());
        Ok(())
    }

    pub fn get_legal_hold(&self, legal_hold_id: &str) -> Result<&LegalHoldData, BackupError> {
        self.legal_holds
            .get(legal_hold_id)
            .ok_or_else(|| BackupError::LegalHoldNotFound(legal_hold_id.to_string()))
    }

    pub fn list_legal_holds(&self) -> Vec<&LegalHoldData> {
        self.legal_holds.values().collect()
    }

    // --- Copy Job operations ---

    pub fn start_copy_job(
        &mut self,
        source_backup_vault_name: &str,
        source_recovery_point_arn: &str,
        destination_backup_vault_arn: &str,
        iam_role_arn: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&CopyJobData, BackupError> {
        let copy_job_id = Uuid::new_v4().to_string();
        let source_vault_arn =
            format!("arn:aws:backup:{region}:{account_id}:backup-vault:{source_backup_vault_name}");
        let dest_rp_id = Uuid::new_v4().to_string();
        let dest_rp_arn =
            format!("arn:aws:backup:{region}:{account_id}:recovery-point:{dest_rp_id}");

        let job = CopyJobData {
            copy_job_id: copy_job_id.clone(),
            source_backup_vault_name: source_backup_vault_name.to_string(),
            source_backup_vault_arn: source_vault_arn,
            source_recovery_point_arn: source_recovery_point_arn.to_string(),
            destination_backup_vault_arn: destination_backup_vault_arn.to_string(),
            destination_recovery_point_arn: dest_rp_arn,
            resource_arn: String::new(),
            resource_type: String::new(),
            iam_role_arn: iam_role_arn.to_string(),
            state: "COMPLETED".to_string(),
            creation_date: Utc::now(),
            completion_date: Some(Utc::now()),
            account_id: account_id.to_string(),
        };

        self.copy_jobs.insert(copy_job_id.clone(), job);
        Ok(self.copy_jobs.get(&copy_job_id).unwrap())
    }

    pub fn describe_copy_job(&self, copy_job_id: &str) -> Result<&CopyJobData, BackupError> {
        self.copy_jobs
            .get(copy_job_id)
            .ok_or_else(|| BackupError::CopyJobNotFound(copy_job_id.to_string()))
    }

    pub fn list_copy_jobs(&self) -> Vec<&CopyJobData> {
        self.copy_jobs.values().collect()
    }

    // --- Restore Job operations ---

    pub fn start_restore_job(
        &mut self,
        recovery_point_arn: &str,
        iam_role_arn: &str,
        resource_type: &str,
        metadata: HashMap<String, String>,
        account_id: &str,
    ) -> Result<&RestoreJobData, BackupError> {
        let restore_job_id = Uuid::new_v4().to_string();

        let job = RestoreJobData {
            restore_job_id: restore_job_id.clone(),
            recovery_point_arn: recovery_point_arn.to_string(),
            resource_type: resource_type.to_string(),
            iam_role_arn: iam_role_arn.to_string(),
            status: "COMPLETED".to_string(),
            creation_date: Utc::now(),
            completion_date: Some(Utc::now()),
            backup_size_in_bytes: 0,
            account_id: account_id.to_string(),
            metadata,
            validation_status: None,
            validation_status_message: None,
        };

        self.restore_jobs.insert(restore_job_id.clone(), job);
        Ok(self.restore_jobs.get(&restore_job_id).unwrap())
    }

    pub fn describe_restore_job(
        &self,
        restore_job_id: &str,
    ) -> Result<&RestoreJobData, BackupError> {
        self.restore_jobs
            .get(restore_job_id)
            .ok_or_else(|| BackupError::RestoreJobNotFound(restore_job_id.to_string()))
    }

    pub fn list_restore_jobs(&self) -> Vec<&RestoreJobData> {
        self.restore_jobs.values().collect()
    }

    pub fn list_restore_jobs_by_recovery_point(&self, resource_arn: &str) -> Vec<&RestoreJobData> {
        self.restore_jobs
            .values()
            .filter(|j| j.recovery_point_arn.contains(resource_arn))
            .collect()
    }

    pub fn put_restore_validation_result(
        &mut self,
        restore_job_id: &str,
        validation_status: &str,
        validation_status_message: Option<&str>,
    ) -> Result<(), BackupError> {
        let job = self
            .restore_jobs
            .get_mut(restore_job_id)
            .ok_or_else(|| BackupError::RestoreJobNotFound(restore_job_id.to_string()))?;
        job.validation_status = Some(validation_status.to_string());
        job.validation_status_message = validation_status_message.map(|s| s.to_string());
        Ok(())
    }

    pub fn get_restore_job_metadata(
        &self,
        restore_job_id: &str,
    ) -> Result<&RestoreJobData, BackupError> {
        self.restore_jobs
            .get(restore_job_id)
            .ok_or_else(|| BackupError::RestoreJobNotFound(restore_job_id.to_string()))
    }

    // --- Restore Testing Plan operations ---

    #[allow(clippy::too_many_arguments)]
    pub fn create_restore_testing_plan(
        &mut self,
        name: &str,
        schedule_expression: &str,
        schedule_expression_timezone: Option<String>,
        start_window_hours: Option<i32>,
        recovery_point_selection: serde_json::Value,
        creator_request_id: Option<String>,
        region: &str,
        account_id: &str,
        tags: HashMap<String, String>,
    ) -> Result<&RestoreTestingPlanData, BackupError> {
        if self.restore_testing_plans.contains_key(name) {
            return Err(BackupError::RestoreTestingPlanAlreadyExists(
                name.to_string(),
            ));
        }

        let arn = format!("arn:aws:backup:{region}:{account_id}:restore-testing-plan:{name}");
        let now = Utc::now();
        let plan = RestoreTestingPlanData {
            restore_testing_plan_name: name.to_string(),
            restore_testing_plan_arn: arn.clone(),
            schedule_expression: schedule_expression.to_string(),
            schedule_expression_timezone,
            start_window_hours,
            recovery_point_selection,
            creator_request_id,
            creation_time: now,
            last_update_time: now,
            tags: tags.clone(),
        };

        if !tags.is_empty() {
            self.resource_tags.insert(arn, tags);
        }

        self.restore_testing_plans.insert(name.to_string(), plan);
        Ok(self.restore_testing_plans.get(name).unwrap())
    }

    pub fn get_restore_testing_plan(
        &self,
        name: &str,
    ) -> Result<&RestoreTestingPlanData, BackupError> {
        self.restore_testing_plans
            .get(name)
            .ok_or_else(|| BackupError::RestoreTestingPlanNotFound(name.to_string()))
    }

    pub fn delete_restore_testing_plan(&mut self, name: &str) -> Result<(), BackupError> {
        if self.restore_testing_plans.remove(name).is_none() {
            return Err(BackupError::RestoreTestingPlanNotFound(name.to_string()));
        }
        // Also remove associated selections
        self.restore_testing_selections
            .retain(|(pn, _), _| pn != name);
        Ok(())
    }

    pub fn list_restore_testing_plans(&self) -> Vec<&RestoreTestingPlanData> {
        self.restore_testing_plans.values().collect()
    }

    pub fn update_restore_testing_plan(
        &mut self,
        name: &str,
        schedule_expression: Option<&str>,
        schedule_expression_timezone: Option<String>,
        start_window_hours: Option<i32>,
        recovery_point_selection: Option<serde_json::Value>,
    ) -> Result<&RestoreTestingPlanData, BackupError> {
        let plan = self
            .restore_testing_plans
            .get_mut(name)
            .ok_or_else(|| BackupError::RestoreTestingPlanNotFound(name.to_string()))?;
        if let Some(se) = schedule_expression {
            plan.schedule_expression = se.to_string();
        }
        if let Some(tz) = schedule_expression_timezone {
            plan.schedule_expression_timezone = Some(tz);
        }
        if let Some(swh) = start_window_hours {
            plan.start_window_hours = Some(swh);
        }
        if let Some(rps) = recovery_point_selection {
            plan.recovery_point_selection = rps;
        }
        plan.last_update_time = Utc::now();
        Ok(self.restore_testing_plans.get(name).unwrap())
    }

    // --- Restore Testing Selection operations ---

    #[allow(clippy::too_many_arguments)]
    pub fn create_restore_testing_selection(
        &mut self,
        plan_name: &str,
        selection_name: &str,
        iam_role_arn: &str,
        protected_resource_type: &str,
        protected_resource_arns: Vec<String>,
        protected_resource_conditions: serde_json::Value,
        restore_metadata_overrides: HashMap<String, String>,
        validation_window_hours: Option<i32>,
        creator_request_id: Option<String>,
    ) -> Result<&RestoreTestingSelectionData, BackupError> {
        let plan = self
            .restore_testing_plans
            .get(plan_name)
            .ok_or_else(|| BackupError::RestoreTestingPlanNotFound(plan_name.to_string()))?;
        let plan_arn = plan.restore_testing_plan_arn.clone();

        let key = (plan_name.to_string(), selection_name.to_string());
        if self.restore_testing_selections.contains_key(&key) {
            return Err(BackupError::RestoreTestingSelectionAlreadyExists(
                selection_name.to_string(),
            ));
        }

        let now = Utc::now();
        let sel = RestoreTestingSelectionData {
            restore_testing_selection_name: selection_name.to_string(),
            restore_testing_plan_name: plan_name.to_string(),
            restore_testing_plan_arn: plan_arn,
            iam_role_arn: iam_role_arn.to_string(),
            protected_resource_type: protected_resource_type.to_string(),
            protected_resource_arns,
            protected_resource_conditions,
            restore_metadata_overrides,
            validation_window_hours,
            creator_request_id,
            creation_time: now,
            last_update_time: now,
        };

        self.restore_testing_selections.insert(key.clone(), sel);
        Ok(self.restore_testing_selections.get(&key).unwrap())
    }

    pub fn get_restore_testing_selection(
        &self,
        plan_name: &str,
        selection_name: &str,
    ) -> Result<&RestoreTestingSelectionData, BackupError> {
        let key = (plan_name.to_string(), selection_name.to_string());
        self.restore_testing_selections
            .get(&key)
            .ok_or_else(|| BackupError::RestoreTestingSelectionNotFound(selection_name.to_string()))
    }

    pub fn delete_restore_testing_selection(
        &mut self,
        plan_name: &str,
        selection_name: &str,
    ) -> Result<(), BackupError> {
        let key = (plan_name.to_string(), selection_name.to_string());
        if self.restore_testing_selections.remove(&key).is_none() {
            return Err(BackupError::RestoreTestingSelectionNotFound(
                selection_name.to_string(),
            ));
        }
        Ok(())
    }

    pub fn list_restore_testing_selections(
        &self,
        plan_name: &str,
    ) -> Vec<&RestoreTestingSelectionData> {
        self.restore_testing_selections
            .values()
            .filter(|s| s.restore_testing_plan_name == plan_name)
            .collect()
    }

    pub fn update_restore_testing_selection(
        &mut self,
        plan_name: &str,
        selection_name: &str,
        iam_role_arn: Option<&str>,
        protected_resource_arns: Option<Vec<String>>,
        protected_resource_conditions: Option<serde_json::Value>,
        restore_metadata_overrides: Option<HashMap<String, String>>,
        validation_window_hours: Option<i32>,
    ) -> Result<&RestoreTestingSelectionData, BackupError> {
        let key = (plan_name.to_string(), selection_name.to_string());
        let sel = self
            .restore_testing_selections
            .get_mut(&key)
            .ok_or_else(|| {
                BackupError::RestoreTestingSelectionNotFound(selection_name.to_string())
            })?;
        if let Some(role) = iam_role_arn {
            sel.iam_role_arn = role.to_string();
        }
        if let Some(arns) = protected_resource_arns {
            sel.protected_resource_arns = arns;
        }
        if let Some(conds) = protected_resource_conditions {
            sel.protected_resource_conditions = conds;
        }
        if let Some(overrides) = restore_metadata_overrides {
            sel.restore_metadata_overrides = overrides;
        }
        if let Some(vwh) = validation_window_hours {
            sel.validation_window_hours = Some(vwh);
        }
        sel.last_update_time = Utc::now();
        Ok(self.restore_testing_selections.get(&key).unwrap())
    }

    // --- UpdateBackupPlan ---

    pub fn update_backup_plan(
        &mut self,
        plan_id: &str,
        backup_plan_json: &serde_json::Value,
    ) -> Result<&BackupPlanData, BackupError> {
        let plan = self
            .backup_plans
            .get_mut(plan_id)
            .ok_or_else(|| BackupError::PlanNotFound(plan_id.to_string()))?;

        // Enrich rules with RuleId if missing
        let enriched_json =
            if let Some(rules_arr) = backup_plan_json.get("Rules").and_then(|v| v.as_array()) {
                let enriched_rules: Vec<serde_json::Value> = rules_arr
                    .iter()
                    .map(|rule| {
                        let mut r = rule.clone();
                        if let serde_json::Value::Object(ref mut map) = r {
                            map.entry("RuleId").or_insert_with(|| {
                                serde_json::Value::String(Uuid::new_v4().to_string())
                            });
                        }
                        r
                    })
                    .collect();
                let mut enriched = backup_plan_json.clone();
                if let serde_json::Value::Object(ref mut map) = enriched {
                    map.insert(
                        "Rules".to_string(),
                        serde_json::Value::Array(enriched_rules),
                    );
                }
                enriched
            } else {
                backup_plan_json.clone()
            };

        plan.backup_plan_json = enriched_json;
        plan.version_id = Uuid::new_v4().to_string();
        if let Some(name) = backup_plan_json
            .get("BackupPlanName")
            .and_then(|v| v.as_str())
        {
            plan.backup_plan_name = name.to_string();
        }

        Ok(self.backup_plans.get(plan_id).unwrap())
    }
}

//! Serde-compatible view types for Backup state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::BackupService;
use crate::state::BackupState;
use crate::types::{
    BackupPlanData, BackupVault, FrameworkData, LegalHoldData, ReportPlanData,
    RestoreTestingPlanData, RestoreTestingSelectionData, TieringConfigData, VaultAccessPolicy,
    VaultNotificationConfig,
};

/// Serializable view of the entire Backup state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BackupStateView {
    /// Backup vaults keyed by vault name.
    #[serde(default)]
    pub vaults: HashMap<String, BackupVaultView>,
    /// Backup plans keyed by plan ID.
    #[serde(default)]
    pub backup_plans: HashMap<String, BackupPlanView>,
    /// Report plans keyed by report plan name.
    #[serde(default)]
    pub report_plans: HashMap<String, ReportPlanView>,
    /// Tags keyed by resource ARN.
    #[serde(default)]
    pub resource_tags: HashMap<String, HashMap<String, String>>,
    /// Audit frameworks keyed by framework name.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub frameworks: HashMap<String, FrameworkView>,
    /// Vault access policies keyed by vault name.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub vault_access_policies: HashMap<String, VaultAccessPolicyView>,
    /// Vault notification configurations keyed by vault name.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub vault_notifications: HashMap<String, VaultNotificationConfigView>,
    /// Tiering configurations keyed by configuration name.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tiering_configs: HashMap<String, TieringConfigView>,
    /// Legal holds keyed by legal_hold_id.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub legal_holds: HashMap<String, LegalHoldView>,
    /// Restore testing plans keyed by plan name.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub restore_testing_plans: HashMap<String, RestoreTestingPlanView>,
    /// Restore testing selections keyed by "plan_name/selection_name".
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub restore_testing_selections: HashMap<String, RestoreTestingSelectionView>,
}

/// Serializable view of a backup vault.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupVaultView {
    pub backup_vault_name: String,
    pub backup_vault_arn: String,
    pub creation_date: String,
    pub number_of_recovery_points: i64,
    pub locked: bool,
    pub min_retention_days: Option<i64>,
    pub max_retention_days: Option<i64>,
    pub lock_date: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of a backup plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupPlanView {
    pub backup_plan_id: String,
    pub backup_plan_arn: String,
    pub backup_plan_name: String,
    pub version_id: String,
    pub creation_date: String,
    pub backup_plan_json: serde_json::Value,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of a report plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportPlanView {
    pub report_plan_name: String,
    pub report_plan_arn: String,
    pub report_plan_description: String,
    pub report_delivery_channel: serde_json::Value,
    pub report_setting: serde_json::Value,
    pub creation_time: String,
    pub deployment_status: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of an audit framework.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkView {
    pub framework_name: String,
    pub framework_arn: String,
    pub framework_description: String,
    pub framework_controls: serde_json::Value,
    pub creation_time: String,
    pub deployment_status: String,
    pub number_of_controls: i32,
}

/// Serializable view of a vault access policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultAccessPolicyView {
    pub backup_vault_name: String,
    pub backup_vault_arn: String,
    pub policy: String,
}

/// Serializable view of a vault notification configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultNotificationConfigView {
    pub backup_vault_name: String,
    pub backup_vault_arn: String,
    pub sns_topic_arn: String,
    #[serde(default)]
    pub backup_vault_events: Vec<String>,
}

/// Serializable view of a tiering configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TieringConfigView {
    pub tiering_configuration_name: String,
    pub tiering_configuration_arn: String,
    pub backup_vault_name: String,
    pub resource_selection: serde_json::Value,
    pub creation_time: String,
    pub last_updated_time: String,
    #[serde(default)]
    pub creator_request_id: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of a legal hold.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalHoldView {
    pub legal_hold_id: String,
    pub legal_hold_arn: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub creation_date: String,
    #[serde(default)]
    pub cancellation_date: Option<String>,
    pub recovery_point_selection: serde_json::Value,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of a restore testing plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreTestingPlanView {
    pub restore_testing_plan_name: String,
    pub restore_testing_plan_arn: String,
    pub schedule_expression: String,
    #[serde(default)]
    pub schedule_expression_timezone: Option<String>,
    #[serde(default)]
    pub start_window_hours: Option<i32>,
    pub recovery_point_selection: serde_json::Value,
    #[serde(default)]
    pub creator_request_id: Option<String>,
    pub creation_time: String,
    pub last_update_time: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of a restore testing selection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreTestingSelectionView {
    pub restore_testing_selection_name: String,
    pub restore_testing_plan_name: String,
    pub restore_testing_plan_arn: String,
    pub iam_role_arn: String,
    pub protected_resource_type: String,
    #[serde(default)]
    pub protected_resource_arns: Vec<String>,
    pub protected_resource_conditions: serde_json::Value,
    #[serde(default)]
    pub restore_metadata_overrides: HashMap<String, String>,
    #[serde(default)]
    pub validation_window_hours: Option<i32>,
    #[serde(default)]
    pub creator_request_id: Option<String>,
    pub creation_time: String,
    pub last_update_time: String,
}

// ---------------------------------------------------------------------------
// From conversions
// ---------------------------------------------------------------------------

impl From<&BackupVault> for BackupVaultView {
    fn from(v: &BackupVault) -> Self {
        BackupVaultView {
            backup_vault_name: v.backup_vault_name.clone(),
            backup_vault_arn: v.backup_vault_arn.clone(),
            creation_date: v.creation_date.to_rfc3339(),
            number_of_recovery_points: v.number_of_recovery_points,
            locked: v.locked,
            min_retention_days: v.min_retention_days,
            max_retention_days: v.max_retention_days,
            lock_date: v.lock_date.as_ref().map(|d| d.to_rfc3339()),
            tags: v.tags.clone(),
        }
    }
}

impl From<&BackupPlanData> for BackupPlanView {
    fn from(p: &BackupPlanData) -> Self {
        BackupPlanView {
            backup_plan_id: p.backup_plan_id.clone(),
            backup_plan_arn: p.backup_plan_arn.clone(),
            backup_plan_name: p.backup_plan_name.clone(),
            version_id: p.version_id.clone(),
            creation_date: p.creation_date.to_rfc3339(),
            backup_plan_json: p.backup_plan_json.clone(),
            tags: p.tags.clone(),
        }
    }
}

impl From<&ReportPlanData> for ReportPlanView {
    fn from(r: &ReportPlanData) -> Self {
        ReportPlanView {
            report_plan_name: r.report_plan_name.clone(),
            report_plan_arn: r.report_plan_arn.clone(),
            report_plan_description: r.report_plan_description.clone(),
            report_delivery_channel: r.report_delivery_channel.clone(),
            report_setting: r.report_setting.clone(),
            creation_time: r.creation_time.to_rfc3339(),
            deployment_status: r.deployment_status.clone(),
            tags: r.tags.clone(),
        }
    }
}

impl From<&FrameworkData> for FrameworkView {
    fn from(f: &FrameworkData) -> Self {
        FrameworkView {
            framework_name: f.framework_name.clone(),
            framework_arn: f.framework_arn.clone(),
            framework_description: f.framework_description.clone(),
            framework_controls: f.framework_controls.clone(),
            creation_time: f.creation_time.to_rfc3339(),
            deployment_status: f.deployment_status.clone(),
            number_of_controls: f.number_of_controls,
        }
    }
}

impl From<&VaultAccessPolicy> for VaultAccessPolicyView {
    fn from(p: &VaultAccessPolicy) -> Self {
        VaultAccessPolicyView {
            backup_vault_name: p.backup_vault_name.clone(),
            backup_vault_arn: p.backup_vault_arn.clone(),
            policy: p.policy.clone(),
        }
    }
}

impl From<&VaultNotificationConfig> for VaultNotificationConfigView {
    fn from(n: &VaultNotificationConfig) -> Self {
        VaultNotificationConfigView {
            backup_vault_name: n.backup_vault_name.clone(),
            backup_vault_arn: n.backup_vault_arn.clone(),
            sns_topic_arn: n.sns_topic_arn.clone(),
            backup_vault_events: n.backup_vault_events.clone(),
        }
    }
}

impl From<&TieringConfigData> for TieringConfigView {
    fn from(t: &TieringConfigData) -> Self {
        TieringConfigView {
            tiering_configuration_name: t.tiering_configuration_name.clone(),
            tiering_configuration_arn: t.tiering_configuration_arn.clone(),
            backup_vault_name: t.backup_vault_name.clone(),
            resource_selection: t.resource_selection.clone(),
            creation_time: t.creation_time.to_rfc3339(),
            last_updated_time: t.last_updated_time.to_rfc3339(),
            creator_request_id: t.creator_request_id.clone(),
            tags: t.tags.clone(),
        }
    }
}

impl From<&LegalHoldData> for LegalHoldView {
    fn from(h: &LegalHoldData) -> Self {
        LegalHoldView {
            legal_hold_id: h.legal_hold_id.clone(),
            legal_hold_arn: h.legal_hold_arn.clone(),
            title: h.title.clone(),
            description: h.description.clone(),
            status: h.status.clone(),
            creation_date: h.creation_date.to_rfc3339(),
            cancellation_date: h.cancellation_date.as_ref().map(|d| d.to_rfc3339()),
            recovery_point_selection: h.recovery_point_selection.clone(),
            tags: h.tags.clone(),
        }
    }
}

impl From<&RestoreTestingPlanData> for RestoreTestingPlanView {
    fn from(p: &RestoreTestingPlanData) -> Self {
        RestoreTestingPlanView {
            restore_testing_plan_name: p.restore_testing_plan_name.clone(),
            restore_testing_plan_arn: p.restore_testing_plan_arn.clone(),
            schedule_expression: p.schedule_expression.clone(),
            schedule_expression_timezone: p.schedule_expression_timezone.clone(),
            start_window_hours: p.start_window_hours,
            recovery_point_selection: p.recovery_point_selection.clone(),
            creator_request_id: p.creator_request_id.clone(),
            creation_time: p.creation_time.to_rfc3339(),
            last_update_time: p.last_update_time.to_rfc3339(),
            tags: p.tags.clone(),
        }
    }
}

impl From<&RestoreTestingSelectionData> for RestoreTestingSelectionView {
    fn from(s: &RestoreTestingSelectionData) -> Self {
        RestoreTestingSelectionView {
            restore_testing_selection_name: s.restore_testing_selection_name.clone(),
            restore_testing_plan_name: s.restore_testing_plan_name.clone(),
            restore_testing_plan_arn: s.restore_testing_plan_arn.clone(),
            iam_role_arn: s.iam_role_arn.clone(),
            protected_resource_type: s.protected_resource_type.clone(),
            protected_resource_arns: s.protected_resource_arns.clone(),
            protected_resource_conditions: s.protected_resource_conditions.clone(),
            restore_metadata_overrides: s.restore_metadata_overrides.clone(),
            validation_window_hours: s.validation_window_hours,
            creator_request_id: s.creator_request_id.clone(),
            creation_time: s.creation_time.to_rfc3339(),
            last_update_time: s.last_update_time.to_rfc3339(),
        }
    }
}

impl From<&BackupState> for BackupStateView {
    fn from(s: &BackupState) -> Self {
        let vaults = s
            .vaults
            .iter()
            .map(|(k, v)| (k.clone(), BackupVaultView::from(v)))
            .collect();
        let backup_plans = s
            .backup_plans
            .iter()
            .map(|(k, v)| (k.clone(), BackupPlanView::from(v)))
            .collect();
        let report_plans = s
            .report_plans
            .iter()
            .map(|(k, v)| (k.clone(), ReportPlanView::from(v)))
            .collect();
        let frameworks = s
            .frameworks
            .iter()
            .map(|(k, v)| (k.clone(), FrameworkView::from(v)))
            .collect();
        let vault_access_policies = s
            .vault_access_policies
            .iter()
            .map(|(k, v)| (k.clone(), VaultAccessPolicyView::from(v)))
            .collect();
        let vault_notifications = s
            .vault_notifications
            .iter()
            .map(|(k, v)| (k.clone(), VaultNotificationConfigView::from(v)))
            .collect();
        let tiering_configs = s
            .tiering_configs
            .iter()
            .map(|(k, v)| (k.clone(), TieringConfigView::from(v)))
            .collect();
        let legal_holds = s
            .legal_holds
            .iter()
            .map(|(k, v)| (k.clone(), LegalHoldView::from(v)))
            .collect();
        let restore_testing_plans = s
            .restore_testing_plans
            .iter()
            .map(|(k, v)| (k.clone(), RestoreTestingPlanView::from(v)))
            .collect();
        let restore_testing_selections = s
            .restore_testing_selections
            .iter()
            .map(|((plan, sel), v)| {
                (
                    format!("{plan}/{sel}"),
                    RestoreTestingSelectionView::from(v),
                )
            })
            .collect();
        BackupStateView {
            vaults,
            backup_plans,
            report_plans,
            resource_tags: s.resource_tags.clone(),
            frameworks,
            vault_access_policies,
            vault_notifications,
            tiering_configs,
            legal_holds,
            restore_testing_plans,
            restore_testing_selections,
        }
    }
}

fn parse_rfc3339_or_now(s: &str) -> chrono::DateTime<chrono::Utc> {
    use chrono::{DateTime, Utc};
    DateTime::parse_from_rfc3339(s)
        .map(|d| d.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

fn parse_rfc3339_opt(s: Option<String>) -> Option<chrono::DateTime<chrono::Utc>> {
    use chrono::{DateTime, Utc};
    s.as_deref()
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|d| d.with_timezone(&Utc))
}

impl From<FrameworkView> for FrameworkData {
    fn from(v: FrameworkView) -> Self {
        FrameworkData {
            framework_name: v.framework_name,
            framework_arn: v.framework_arn,
            framework_description: v.framework_description,
            framework_controls: v.framework_controls,
            creation_time: parse_rfc3339_or_now(&v.creation_time),
            deployment_status: v.deployment_status,
            number_of_controls: v.number_of_controls,
        }
    }
}

impl From<VaultAccessPolicyView> for VaultAccessPolicy {
    fn from(v: VaultAccessPolicyView) -> Self {
        VaultAccessPolicy {
            backup_vault_name: v.backup_vault_name,
            backup_vault_arn: v.backup_vault_arn,
            policy: v.policy,
        }
    }
}

impl From<VaultNotificationConfigView> for VaultNotificationConfig {
    fn from(v: VaultNotificationConfigView) -> Self {
        VaultNotificationConfig {
            backup_vault_name: v.backup_vault_name,
            backup_vault_arn: v.backup_vault_arn,
            sns_topic_arn: v.sns_topic_arn,
            backup_vault_events: v.backup_vault_events,
        }
    }
}

impl From<TieringConfigView> for TieringConfigData {
    fn from(v: TieringConfigView) -> Self {
        TieringConfigData {
            tiering_configuration_name: v.tiering_configuration_name,
            tiering_configuration_arn: v.tiering_configuration_arn,
            backup_vault_name: v.backup_vault_name,
            resource_selection: v.resource_selection,
            creation_time: parse_rfc3339_or_now(&v.creation_time),
            last_updated_time: parse_rfc3339_or_now(&v.last_updated_time),
            creator_request_id: v.creator_request_id,
            tags: v.tags,
        }
    }
}

impl From<LegalHoldView> for LegalHoldData {
    fn from(v: LegalHoldView) -> Self {
        LegalHoldData {
            legal_hold_id: v.legal_hold_id,
            legal_hold_arn: v.legal_hold_arn,
            title: v.title,
            description: v.description,
            status: v.status,
            creation_date: parse_rfc3339_or_now(&v.creation_date),
            cancellation_date: parse_rfc3339_opt(v.cancellation_date),
            recovery_point_selection: v.recovery_point_selection,
            tags: v.tags,
        }
    }
}

impl From<RestoreTestingPlanView> for RestoreTestingPlanData {
    fn from(v: RestoreTestingPlanView) -> Self {
        RestoreTestingPlanData {
            restore_testing_plan_name: v.restore_testing_plan_name,
            restore_testing_plan_arn: v.restore_testing_plan_arn,
            schedule_expression: v.schedule_expression,
            schedule_expression_timezone: v.schedule_expression_timezone,
            start_window_hours: v.start_window_hours,
            recovery_point_selection: v.recovery_point_selection,
            creator_request_id: v.creator_request_id,
            creation_time: parse_rfc3339_or_now(&v.creation_time),
            last_update_time: parse_rfc3339_or_now(&v.last_update_time),
            tags: v.tags,
        }
    }
}

impl From<RestoreTestingSelectionView> for RestoreTestingSelectionData {
    fn from(v: RestoreTestingSelectionView) -> Self {
        RestoreTestingSelectionData {
            restore_testing_selection_name: v.restore_testing_selection_name,
            restore_testing_plan_name: v.restore_testing_plan_name,
            restore_testing_plan_arn: v.restore_testing_plan_arn,
            iam_role_arn: v.iam_role_arn,
            protected_resource_type: v.protected_resource_type,
            protected_resource_arns: v.protected_resource_arns,
            protected_resource_conditions: v.protected_resource_conditions,
            restore_metadata_overrides: v.restore_metadata_overrides,
            validation_window_hours: v.validation_window_hours,
            creator_request_id: v.creator_request_id,
            creation_time: parse_rfc3339_or_now(&v.creation_time),
            last_update_time: parse_rfc3339_or_now(&v.last_update_time),
        }
    }
}

// ---------------------------------------------------------------------------
// StatefulService implementation
// ---------------------------------------------------------------------------

impl StatefulService for BackupService {
    type StateView = BackupStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        BackupStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        use chrono::{DateTime, Utc};

        let mut new_state = BackupState::default();

        for (name, vv) in view.vaults {
            let creation_date = DateTime::parse_from_rfc3339(&vv.creation_date)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let lock_date = vv
                .lock_date
                .as_deref()
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                .map(|d| d.with_timezone(&Utc));
            new_state.vaults.insert(
                name,
                BackupVault {
                    backup_vault_name: vv.backup_vault_name,
                    backup_vault_arn: vv.backup_vault_arn,
                    creation_date,
                    number_of_recovery_points: vv.number_of_recovery_points,
                    locked: vv.locked,
                    min_retention_days: vv.min_retention_days,
                    max_retention_days: vv.max_retention_days,
                    lock_date,
                    tags: vv.tags,
                },
            );
        }

        for (id, pv) in view.backup_plans {
            let creation_date = DateTime::parse_from_rfc3339(&pv.creation_date)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            new_state.backup_plans.insert(
                id,
                BackupPlanData {
                    backup_plan_id: pv.backup_plan_id,
                    backup_plan_arn: pv.backup_plan_arn,
                    backup_plan_name: pv.backup_plan_name,
                    version_id: pv.version_id,
                    creation_date,
                    backup_plan_json: pv.backup_plan_json,
                    tags: pv.tags,
                },
            );
        }

        for (name, rv) in view.report_plans {
            let creation_time = DateTime::parse_from_rfc3339(&rv.creation_time)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            new_state.report_plans.insert(
                name,
                ReportPlanData {
                    report_plan_name: rv.report_plan_name,
                    report_plan_arn: rv.report_plan_arn,
                    report_plan_description: rv.report_plan_description,
                    report_delivery_channel: rv.report_delivery_channel,
                    report_setting: rv.report_setting,
                    creation_time,
                    deployment_status: rv.deployment_status,
                    tags: rv.tags,
                },
            );
        }

        new_state.resource_tags = view.resource_tags;

        for (k, v) in view.frameworks {
            new_state.frameworks.insert(k, FrameworkData::from(v));
        }
        for (k, v) in view.vault_access_policies {
            new_state
                .vault_access_policies
                .insert(k, VaultAccessPolicy::from(v));
        }
        for (k, v) in view.vault_notifications {
            new_state
                .vault_notifications
                .insert(k, VaultNotificationConfig::from(v));
        }
        for (k, v) in view.tiering_configs {
            new_state
                .tiering_configs
                .insert(k, TieringConfigData::from(v));
        }
        for (k, v) in view.legal_holds {
            new_state.legal_holds.insert(k, LegalHoldData::from(v));
        }
        for (k, v) in view.restore_testing_plans {
            new_state
                .restore_testing_plans
                .insert(k, RestoreTestingPlanData::from(v));
        }
        for (key, v) in view.restore_testing_selections {
            // Prefer parsing the composite key, but fall back to the embedded field values.
            let (plan, sel) = match key.split_once('/') {
                Some((p, s)) => (p.to_string(), s.to_string()),
                None => (
                    v.restore_testing_plan_name.clone(),
                    v.restore_testing_selection_name.clone(),
                ),
            };
            new_state
                .restore_testing_selections
                .insert((plan, sel), RestoreTestingSelectionData::from(v));
        }

        {
            let state = self.state.get(account_id, region);
            *state.write().await = new_state;
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        use chrono::{DateTime, Utc};

        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;

            for (name, vv) in view.vaults {
                let creation_date = DateTime::parse_from_rfc3339(&vv.creation_date)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());
                let lock_date = vv
                    .lock_date
                    .as_deref()
                    .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                    .map(|d| d.with_timezone(&Utc));
                guard.vaults.insert(
                    name,
                    BackupVault {
                        backup_vault_name: vv.backup_vault_name,
                        backup_vault_arn: vv.backup_vault_arn,
                        creation_date,
                        number_of_recovery_points: vv.number_of_recovery_points,
                        locked: vv.locked,
                        min_retention_days: vv.min_retention_days,
                        max_retention_days: vv.max_retention_days,
                        lock_date,
                        tags: vv.tags,
                    },
                );
            }

            for (id, pv) in view.backup_plans {
                let creation_date = DateTime::parse_from_rfc3339(&pv.creation_date)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());
                guard.backup_plans.insert(
                    id,
                    BackupPlanData {
                        backup_plan_id: pv.backup_plan_id,
                        backup_plan_arn: pv.backup_plan_arn,
                        backup_plan_name: pv.backup_plan_name,
                        version_id: pv.version_id,
                        creation_date,
                        backup_plan_json: pv.backup_plan_json,
                        tags: pv.tags,
                    },
                );
            }

            for (name, rv) in view.report_plans {
                let creation_time = DateTime::parse_from_rfc3339(&rv.creation_time)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());
                guard.report_plans.insert(
                    name,
                    ReportPlanData {
                        report_plan_name: rv.report_plan_name,
                        report_plan_arn: rv.report_plan_arn,
                        report_plan_description: rv.report_plan_description,
                        report_delivery_channel: rv.report_delivery_channel,
                        report_setting: rv.report_setting,
                        creation_time,
                        deployment_status: rv.deployment_status,
                        tags: rv.tags,
                    },
                );
            }

            for (arn, tags) in view.resource_tags {
                guard.resource_tags.entry(arn).or_default().extend(tags);
            }

            for (k, v) in view.frameworks {
                guard.frameworks.insert(k, FrameworkData::from(v));
            }
            for (k, v) in view.vault_access_policies {
                guard
                    .vault_access_policies
                    .insert(k, VaultAccessPolicy::from(v));
            }
            for (k, v) in view.vault_notifications {
                guard
                    .vault_notifications
                    .insert(k, VaultNotificationConfig::from(v));
            }
            for (k, v) in view.tiering_configs {
                guard.tiering_configs.insert(k, TieringConfigData::from(v));
            }
            for (k, v) in view.legal_holds {
                guard.legal_holds.insert(k, LegalHoldData::from(v));
            }
            for (k, v) in view.restore_testing_plans {
                guard
                    .restore_testing_plans
                    .insert(k, RestoreTestingPlanData::from(v));
            }
            for (key, v) in view.restore_testing_selections {
                let (plan, sel) = match key.split_once('/') {
                    Some((p, s)) => (p.to_string(), s.to_string()),
                    None => (
                        v.restore_testing_plan_name.clone(),
                        v.restore_testing_selection_name.clone(),
                    ),
                };
                guard
                    .restore_testing_selections
                    .insert((plan, sel), RestoreTestingSelectionData::from(v));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

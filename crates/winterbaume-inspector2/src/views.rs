//! Serde-compatible view types for Inspector2 state snapshots.

use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::Inspector2Service;
use crate::state::{
    CisScanConfigRecord, Ec2DeepInspectionConfig, FindingsReportRecord, Inspector2State,
    SbomExportRecord,
};
use crate::types::{
    AutoEnableConfig, CodeSecurityIntegration, CodeSecurityScanConfig,
    CodeSecurityScanConfigAssociation, DelegatedAdminRecord, Finding, InspectorFilter,
    MemberEc2DeepInspectionStatus, MemberRecord,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Inspector2StateView {
    /// Per-account enabled resource types ( account id -> set of types ).
    #[serde(default)]
    pub enabled_resource_types: HashMap<String, HashSet<String>>,
    #[serde(default)]
    pub findings: Vec<FindingView>,
    #[serde(default)]
    pub members: HashMap<String, MemberView>,
    #[serde(default)]
    pub filters: HashMap<String, FilterView>,
    #[serde(default)]
    pub delegated_admin_accounts: HashMap<String, DelegatedAdminView>,
    pub auto_enable: Option<AutoEnableView>,
    #[serde(default)]
    pub resource_tags: HashMap<String, HashMap<String, String>>,
    #[serde(default)]
    pub ec2_deep_inspection_activate: bool,
    #[serde(default)]
    pub ec2_deep_inspection_package_paths: Vec<String>,
    #[serde(default)]
    pub org_ec2_deep_inspection_package_paths: Vec<String>,
    #[serde(default)]
    pub ec2_scan_mode: String,
    #[serde(default)]
    pub ecr_rescan_duration: String,
    #[serde(default)]
    pub encryption_key: String,
    #[serde(default)]
    pub findings_reports: HashMap<String, FindingsReportView>,
    #[serde(default)]
    pub sbom_exports: HashMap<String, SbomExportView>,
    #[serde(default)]
    pub cis_scan_configs: HashMap<String, CisScanConfigView>,
    #[serde(default)]
    pub code_security_integrations: HashMap<String, CodeSecurityIntegrationView>,
    #[serde(default)]
    pub code_security_scan_configs: HashMap<String, CodeSecurityScanConfigView>,
    #[serde(default)]
    pub code_security_scan_config_associations: Vec<CodeSecurityScanConfigAssociationView>,
    #[serde(default)]
    pub member_ec2_deep_inspection_status: HashMap<String, MemberEc2DeepInspectionStatusView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindingView {
    pub finding_arn: String,
    pub aws_account_id: String,
    pub description: String,
    pub first_observed_at: String,
    pub last_observed_at: String,
    pub updated_at: String,
    pub severity: String,
    pub status: String,
    pub title: String,
    pub finding_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberView {
    pub account_id: String,
    pub relationship_status: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterView {
    pub arn: String,
    pub name: String,
    pub action: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub owner_id: String,
    pub tags: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegatedAdminView {
    pub account_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoEnableView {
    pub ec2: bool,
    pub ecr: bool,
    pub lambda: Option<bool>,
    pub lambda_code: Option<bool>,
    pub code_repository: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindingsReportView {
    pub report_id: String,
    pub status: String,
    pub report_format: String,
    pub s3_bucket_name: String,
    pub s3_key_prefix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SbomExportView {
    pub report_id: String,
    pub status: String,
    pub report_format: String,
    pub s3_bucket_name: String,
    pub s3_key_prefix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CisScanConfigView {
    pub scan_configuration_arn: String,
    pub scan_name: String,
    pub owner_id: String,
    pub tags: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSecurityIntegrationView {
    pub integration_arn: String,
    pub name: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSecurityScanConfigView {
    pub scan_configuration_arn: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSecurityScanConfigAssociationView {
    pub scan_configuration_arn: String,
    pub resource_id: String,
    pub status: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberEc2DeepInspectionStatusView {
    pub account_id: String,
    pub activate_deep_inspection: bool,
    pub status: String,
}

fn parse_dt(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

// --- From internal types to view types ---

impl From<&FindingsReportRecord> for FindingsReportView {
    fn from(r: &FindingsReportRecord) -> Self {
        FindingsReportView {
            report_id: r.report_id.clone(),
            status: r.status.clone(),
            report_format: r.report_format.clone(),
            s3_bucket_name: r.s3_bucket_name.clone(),
            s3_key_prefix: r.s3_key_prefix.clone(),
        }
    }
}

impl From<FindingsReportView> for FindingsReportRecord {
    fn from(v: FindingsReportView) -> Self {
        FindingsReportRecord {
            report_id: v.report_id,
            status: v.status,
            report_format: v.report_format,
            s3_bucket_name: v.s3_bucket_name,
            s3_key_prefix: v.s3_key_prefix,
        }
    }
}

impl From<&SbomExportRecord> for SbomExportView {
    fn from(r: &SbomExportRecord) -> Self {
        SbomExportView {
            report_id: r.report_id.clone(),
            status: r.status.clone(),
            report_format: r.report_format.clone(),
            s3_bucket_name: r.s3_bucket_name.clone(),
            s3_key_prefix: r.s3_key_prefix.clone(),
        }
    }
}

impl From<SbomExportView> for SbomExportRecord {
    fn from(v: SbomExportView) -> Self {
        SbomExportRecord {
            report_id: v.report_id,
            status: v.status,
            report_format: v.report_format,
            s3_bucket_name: v.s3_bucket_name,
            s3_key_prefix: v.s3_key_prefix,
        }
    }
}

impl From<&CisScanConfigRecord> for CisScanConfigView {
    fn from(r: &CisScanConfigRecord) -> Self {
        CisScanConfigView {
            scan_configuration_arn: r.scan_configuration_arn.clone(),
            scan_name: r.scan_name.clone(),
            owner_id: r.owner_id.clone(),
            tags: r.tags.clone(),
        }
    }
}

impl From<CisScanConfigView> for CisScanConfigRecord {
    fn from(v: CisScanConfigView) -> Self {
        CisScanConfigRecord {
            scan_configuration_arn: v.scan_configuration_arn,
            scan_name: v.scan_name,
            owner_id: v.owner_id,
            tags: v.tags,
        }
    }
}

impl From<&Inspector2State> for Inspector2StateView {
    fn from(state: &Inspector2State) -> Self {
        Inspector2StateView {
            enabled_resource_types: state.enabled_resource_types.clone(),
            findings: state.findings.iter().map(FindingView::from).collect(),
            members: state
                .members
                .iter()
                .map(|(k, v)| (k.clone(), MemberView::from(v)))
                .collect(),
            filters: state
                .filters
                .iter()
                .map(|(k, v)| (k.clone(), FilterView::from(v)))
                .collect(),
            delegated_admin_accounts: state
                .delegated_admin_accounts
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        DelegatedAdminView {
                            account_id: v.account_id.clone(),
                            status: v.status.clone(),
                        },
                    )
                })
                .collect(),
            auto_enable: state.auto_enable.as_ref().map(AutoEnableView::from),
            resource_tags: state.resource_tags.clone(),
            ec2_deep_inspection_activate: state.ec2_deep_inspection.activate_deep_inspection,
            ec2_deep_inspection_package_paths: state.ec2_deep_inspection.package_paths.clone(),
            org_ec2_deep_inspection_package_paths: state
                .org_ec2_deep_inspection_package_paths
                .clone(),
            ec2_scan_mode: state.ec2_scan_mode.clone(),
            ecr_rescan_duration: state.ecr_rescan_duration.clone(),
            encryption_key: state.encryption_key.clone(),
            findings_reports: state
                .findings_reports
                .iter()
                .map(|(k, v)| (k.clone(), FindingsReportView::from(v)))
                .collect(),
            sbom_exports: state
                .sbom_exports
                .iter()
                .map(|(k, v)| (k.clone(), SbomExportView::from(v)))
                .collect(),
            cis_scan_configs: state
                .cis_scan_configs
                .iter()
                .map(|(k, v)| (k.clone(), CisScanConfigView::from(v)))
                .collect(),
            code_security_integrations: state
                .code_security_integrations
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        CodeSecurityIntegrationView {
                            integration_arn: v.integration_arn.clone(),
                            name: v.name.clone(),
                            status: v.status.clone(),
                            created_at: v.created_at.to_rfc3339(),
                            updated_at: v.updated_at.to_rfc3339(),
                        },
                    )
                })
                .collect(),
            code_security_scan_configs: state
                .code_security_scan_configs
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        CodeSecurityScanConfigView {
                            scan_configuration_arn: v.scan_configuration_arn.clone(),
                            name: v.name.clone(),
                            created_at: v.created_at.to_rfc3339(),
                            updated_at: v.updated_at.to_rfc3339(),
                        },
                    )
                })
                .collect(),
            code_security_scan_config_associations: state
                .code_security_scan_config_associations
                .values()
                .map(|a| CodeSecurityScanConfigAssociationView {
                    scan_configuration_arn: a.scan_configuration_arn.clone(),
                    resource_id: a.resource_id.clone(),
                    status: a.status.clone(),
                    updated_at: a.updated_at.to_rfc3339(),
                })
                .collect(),
            member_ec2_deep_inspection_status: state
                .member_ec2_deep_inspection_status
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        MemberEc2DeepInspectionStatusView {
                            account_id: v.account_id.clone(),
                            activate_deep_inspection: v.activate_deep_inspection,
                            status: v.status.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

impl From<&Finding> for FindingView {
    fn from(f: &Finding) -> Self {
        FindingView {
            finding_arn: f.finding_arn.clone(),
            aws_account_id: f.aws_account_id.clone(),
            description: f.description.clone(),
            first_observed_at: f.first_observed_at.to_rfc3339(),
            last_observed_at: f.last_observed_at.to_rfc3339(),
            updated_at: f.updated_at.to_rfc3339(),
            severity: f.severity.clone(),
            status: f.status.clone(),
            title: f.title.clone(),
            finding_type: f.finding_type.clone(),
        }
    }
}

impl From<&MemberRecord> for MemberView {
    fn from(m: &MemberRecord) -> Self {
        MemberView {
            account_id: m.account_id.clone(),
            relationship_status: m.relationship_status.clone(),
            updated_at: m.updated_at.to_rfc3339(),
        }
    }
}

impl From<&InspectorFilter> for FilterView {
    fn from(f: &InspectorFilter) -> Self {
        FilterView {
            arn: f.arn.clone(),
            name: f.name.clone(),
            action: f.action.clone(),
            description: f.description.clone(),
            created_at: f.created_at.to_rfc3339(),
            updated_at: f.updated_at.to_rfc3339(),
            owner_id: f.owner_id.clone(),
            tags: f.tags.clone(),
        }
    }
}

impl From<&AutoEnableConfig> for AutoEnableView {
    fn from(a: &AutoEnableConfig) -> Self {
        AutoEnableView {
            ec2: a.ec2,
            ecr: a.ecr,
            lambda: a.lambda,
            lambda_code: a.lambda_code,
            code_repository: a.code_repository,
        }
    }
}

// --- From view types to internal types ---

impl From<Inspector2StateView> for Inspector2State {
    fn from(view: Inspector2StateView) -> Self {
        Inspector2State {
            enabled_resource_types: view.enabled_resource_types,
            findings: view.findings.into_iter().map(Finding::from).collect(),
            members: view
                .members
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        MemberRecord {
                            account_id: v.account_id,
                            relationship_status: v.relationship_status,
                            updated_at: parse_dt(&v.updated_at),
                        },
                    )
                })
                .collect(),
            filters: view
                .filters
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        InspectorFilter {
                            arn: v.arn,
                            name: v.name,
                            action: v.action,
                            description: v.description,
                            created_at: parse_dt(&v.created_at),
                            updated_at: parse_dt(&v.updated_at),
                            owner_id: v.owner_id,
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            delegated_admin_accounts: view
                .delegated_admin_accounts
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        DelegatedAdminRecord {
                            account_id: v.account_id,
                            status: v.status,
                        },
                    )
                })
                .collect(),
            auto_enable: view.auto_enable.map(|a| AutoEnableConfig {
                ec2: a.ec2,
                ecr: a.ecr,
                lambda: a.lambda,
                lambda_code: a.lambda_code,
                code_repository: a.code_repository,
            }),
            resource_tags: view.resource_tags,
            ec2_deep_inspection: Ec2DeepInspectionConfig {
                activate_deep_inspection: view.ec2_deep_inspection_activate,
                package_paths: view.ec2_deep_inspection_package_paths,
            },
            org_ec2_deep_inspection_package_paths: view.org_ec2_deep_inspection_package_paths,
            ec2_scan_mode: view.ec2_scan_mode,
            ecr_rescan_duration: view.ecr_rescan_duration,
            encryption_key: view.encryption_key,
            findings_reports: view
                .findings_reports
                .into_iter()
                .map(|(k, v)| (k, FindingsReportRecord::from(v)))
                .collect(),
            sbom_exports: view
                .sbom_exports
                .into_iter()
                .map(|(k, v)| (k, SbomExportRecord::from(v)))
                .collect(),
            cis_scan_configs: view
                .cis_scan_configs
                .into_iter()
                .map(|(k, v)| (k, CisScanConfigRecord::from(v)))
                .collect(),
            code_security_integrations: view
                .code_security_integrations
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        CodeSecurityIntegration {
                            integration_arn: v.integration_arn,
                            name: v.name,
                            status: v.status,
                            created_at: parse_dt(&v.created_at),
                            updated_at: parse_dt(&v.updated_at),
                        },
                    )
                })
                .collect(),
            code_security_scan_configs: view
                .code_security_scan_configs
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        CodeSecurityScanConfig {
                            scan_configuration_arn: v.scan_configuration_arn,
                            name: v.name,
                            created_at: parse_dt(&v.created_at),
                            updated_at: parse_dt(&v.updated_at),
                        },
                    )
                })
                .collect(),
            code_security_scan_config_associations: view
                .code_security_scan_config_associations
                .into_iter()
                .map(|a| {
                    let key = (a.scan_configuration_arn.clone(), a.resource_id.clone());
                    (
                        key,
                        CodeSecurityScanConfigAssociation {
                            scan_configuration_arn: a.scan_configuration_arn,
                            resource_id: a.resource_id,
                            status: a.status,
                            updated_at: parse_dt(&a.updated_at),
                        },
                    )
                })
                .collect(),
            member_ec2_deep_inspection_status: view
                .member_ec2_deep_inspection_status
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        MemberEc2DeepInspectionStatus {
                            account_id: v.account_id,
                            activate_deep_inspection: v.activate_deep_inspection,
                            status: v.status,
                        },
                    )
                })
                .collect(),
        }
    }
}

impl From<FindingView> for Finding {
    fn from(f: FindingView) -> Self {
        Finding {
            finding_arn: f.finding_arn,
            aws_account_id: f.aws_account_id,
            description: f.description,
            first_observed_at: parse_dt(&f.first_observed_at),
            last_observed_at: parse_dt(&f.last_observed_at),
            updated_at: parse_dt(&f.updated_at),
            severity: f.severity,
            status: f.status,
            title: f.title,
            finding_type: f.finding_type,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for Inspector2Service {
    type StateView = Inspector2StateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        Inspector2StateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = Inspector2State::from(view);
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
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (acct, rts) in view.enabled_resource_types {
                guard
                    .enabled_resource_types
                    .entry(acct)
                    .or_default()
                    .extend(rts);
            }
            for f in view.findings {
                guard.findings.push(Finding::from(f));
            }
            for (k, v) in view.members {
                guard.members.insert(
                    k,
                    MemberRecord {
                        account_id: v.account_id,
                        relationship_status: v.relationship_status,
                        updated_at: parse_dt(&v.updated_at),
                    },
                );
            }
            for (k, v) in view.filters {
                guard.filters.insert(
                    k,
                    InspectorFilter {
                        arn: v.arn,
                        name: v.name,
                        action: v.action,
                        description: v.description,
                        created_at: parse_dt(&v.created_at),
                        updated_at: parse_dt(&v.updated_at),
                        owner_id: v.owner_id,
                        tags: v.tags,
                    },
                );
            }
            for (k, v) in view.delegated_admin_accounts {
                guard.delegated_admin_accounts.insert(
                    k,
                    DelegatedAdminRecord {
                        account_id: v.account_id,
                        status: v.status,
                    },
                );
            }
            if let Some(a) = view.auto_enable {
                guard.auto_enable = Some(AutoEnableConfig {
                    ec2: a.ec2,
                    ecr: a.ecr,
                    lambda: a.lambda,
                    lambda_code: a.lambda_code,
                    code_repository: a.code_repository,
                });
            }
            for (k, v) in view.resource_tags {
                guard.resource_tags.insert(k, v);
            }
            if view.ec2_deep_inspection_activate {
                guard.ec2_deep_inspection.activate_deep_inspection =
                    view.ec2_deep_inspection_activate;
            }
            if !view.ec2_deep_inspection_package_paths.is_empty() {
                guard.ec2_deep_inspection.package_paths = view.ec2_deep_inspection_package_paths;
            }
            if !view.org_ec2_deep_inspection_package_paths.is_empty() {
                guard.org_ec2_deep_inspection_package_paths =
                    view.org_ec2_deep_inspection_package_paths;
            }
            if !view.ec2_scan_mode.is_empty() {
                guard.ec2_scan_mode = view.ec2_scan_mode;
            }
            if !view.ecr_rescan_duration.is_empty() {
                guard.ecr_rescan_duration = view.ecr_rescan_duration;
            }
            if !view.encryption_key.is_empty() {
                guard.encryption_key = view.encryption_key;
            }
            for (k, v) in view.findings_reports {
                guard
                    .findings_reports
                    .insert(k, FindingsReportRecord::from(v));
            }
            for (k, v) in view.sbom_exports {
                guard.sbom_exports.insert(k, SbomExportRecord::from(v));
            }
            for (k, v) in view.cis_scan_configs {
                guard
                    .cis_scan_configs
                    .insert(k, CisScanConfigRecord::from(v));
            }
            for (k, v) in view.code_security_integrations {
                guard.code_security_integrations.insert(
                    k,
                    CodeSecurityIntegration {
                        integration_arn: v.integration_arn,
                        name: v.name,
                        status: v.status,
                        created_at: parse_dt(&v.created_at),
                        updated_at: parse_dt(&v.updated_at),
                    },
                );
            }
            for (k, v) in view.code_security_scan_configs {
                guard.code_security_scan_configs.insert(
                    k,
                    CodeSecurityScanConfig {
                        scan_configuration_arn: v.scan_configuration_arn,
                        name: v.name,
                        created_at: parse_dt(&v.created_at),
                        updated_at: parse_dt(&v.updated_at),
                    },
                );
            }
            for a in view.code_security_scan_config_associations {
                let key = (a.scan_configuration_arn.clone(), a.resource_id.clone());
                guard.code_security_scan_config_associations.insert(
                    key,
                    CodeSecurityScanConfigAssociation {
                        scan_configuration_arn: a.scan_configuration_arn,
                        resource_id: a.resource_id,
                        status: a.status,
                        updated_at: parse_dt(&a.updated_at),
                    },
                );
            }
            for (k, v) in view.member_ec2_deep_inspection_status {
                guard.member_ec2_deep_inspection_status.insert(
                    k,
                    MemberEc2DeepInspectionStatus {
                        account_id: v.account_id,
                        activate_deep_inspection: v.activate_deep_inspection,
                        status: v.status,
                    },
                );
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

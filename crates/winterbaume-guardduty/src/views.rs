//! Serde-compatible view types for GuardDuty state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::GuardDutyService;
use crate::state::GuardDutyState;
use crate::types::{
    AdminAccount, Condition, CoverageRecord, DataSourcesConfig, Detector,
    DetectorAdditionalConfiguration, DetectorFeature, Filter, FindingCriteria, Invitation, IpSet,
    MalwareProtectionPlan, MalwareProtectionPlanActions, MalwareProtectionPlanResource,
    MalwareScanRecord, MalwareScanSettings, Member, MemberDetectorAdditionalConfig,
    MemberDetectorFeature, OrgConfig, PublishingDestination, StoredFinding, StoredMalwareScan,
    ThreatEntitySet, ThreatIntelSet, TrustedEntitySet,
};

/// Serializable view of the entire GuardDuty state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GuardDutyStateView {
    /// Detectors keyed by detector ID.
    #[serde(default)]
    pub detectors: HashMap<String, DetectorView>,
    /// Organization admin accounts.
    #[serde(default)]
    pub admin_accounts: Vec<AdminAccountView>,
    /// Per-resource tags keyed by resource ARN.
    #[serde(default)]
    pub resource_tags: HashMap<String, HashMap<String, String>>,
    /// Malware scan records keyed by scan ID.
    #[serde(default)]
    pub malware_scans: HashMap<String, MalwareScanRecordView>,
    /// Publishing destinations keyed by detector_id then destination_id.
    #[serde(default)]
    pub publishing_destinations: HashMap<String, HashMap<String, PublishingDestinationView>>,
    /// Malware protection plans keyed by plan ID.
    #[serde(default)]
    pub malware_protection_plans: HashMap<String, MalwareProtectionPlanView>,
    /// Pending invitations keyed by account ID.
    #[serde(default)]
    pub invitations: HashMap<String, InvitationView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DetectorView {
    pub detector_id: String,
    pub status: String,
    pub finding_publishing_frequency: String,
    pub created_at: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    /// Filters keyed by filter name.
    #[serde(default)]
    pub filters: HashMap<String, FilterView>,
    pub data_sources: Option<DataSourcesConfigView>,
    pub features: Option<Vec<DetectorFeatureView>>,
    #[serde(default)]
    pub ip_sets: HashMap<String, IpSetView>,
    #[serde(default)]
    pub threat_intel_sets: HashMap<String, ThreatIntelSetView>,
    #[serde(default)]
    pub threat_entity_sets: HashMap<String, ThreatEntitySetView>,
    #[serde(default)]
    pub trusted_entity_sets: HashMap<String, TrustedEntitySetView>,
    /// Findings keyed by finding ID.
    #[serde(default)]
    pub findings: HashMap<String, StoredFindingView>,
    /// The administrator account ID associated with this detector.
    #[serde(default)]
    pub administrator_account_id: Option<String>,
    /// The master account ID associated with this detector (legacy).
    #[serde(default)]
    pub master_account_id: Option<String>,
    /// Coverage records for this detector.
    #[serde(default)]
    pub coverage_records: Vec<CoverageRecordView>,
    /// Members keyed by account ID.
    #[serde(default)]
    pub members: HashMap<String, MemberView>,
    /// Malware scan settings for this detector.
    #[serde(default)]
    pub malware_scan_settings: MalwareScanSettingsView,
    /// Organization configuration for this detector.
    #[serde(default)]
    pub org_config: OrgConfigView,
    /// Per-detector malware scans keyed by scan ID.
    #[serde(default)]
    pub malware_scans: HashMap<String, StoredMalwareScanView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredFindingView {
    pub id: String,
    pub arn: String,
    pub account_id: String,
    pub region: String,
    pub r#type: String,
    pub severity: f64,
    pub title: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
    pub archived: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterView {
    pub name: String,
    pub description: String,
    pub action: String,
    pub rank: i32,
    pub finding_criteria: FindingCriteriaView,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FindingCriteriaView {
    #[serde(default)]
    pub criterion: HashMap<String, ConditionView>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConditionView {
    pub eq: Option<Vec<String>>,
    pub neq: Option<Vec<String>>,
    pub gt: Option<i64>,
    pub gte: Option<i64>,
    pub lt: Option<i64>,
    pub lte: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataSourcesConfigView {
    pub s3_logs_enabled: Option<bool>,
    pub kubernetes_audit_logs_enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectorFeatureView {
    pub name: String,
    pub status: String,
    #[serde(default)]
    pub additional_configuration: Vec<DetectorAdditionalConfigurationView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectorAdditionalConfigurationView {
    pub name: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminAccountView {
    pub admin_account_id: String,
    pub admin_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpSetView {
    pub ip_set_id: String,
    pub name: String,
    pub format: String,
    pub location: String,
    pub status: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIntelSetView {
    pub threat_intel_set_id: String,
    pub name: String,
    pub format: String,
    pub location: String,
    pub status: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatEntitySetView {
    pub threat_entity_set_id: String,
    pub name: String,
    pub format: String,
    pub location: String,
    pub status: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustedEntitySetView {
    pub trusted_entity_set_id: String,
    pub name: String,
    pub format: String,
    pub location: String,
    pub status: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageRecordView {
    pub account_id: String,
    pub detector_id: String,
    pub coverage_status: String,
    pub resource_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MalwareScanRecordView {
    pub scan_id: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberView {
    pub account_id: String,
    pub email: String,
    pub relationship_status: String,
    #[serde(default)]
    pub invited_at: Option<String>,
    pub updated_at: String,
    #[serde(default)]
    pub detector_features: Vec<MemberDetectorFeatureView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberDetectorFeatureView {
    pub name: String,
    pub status: String,
    #[serde(default)]
    pub additional_configuration: Vec<MemberDetectorAdditionalConfigView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberDetectorAdditionalConfigView {
    pub name: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MalwareScanSettingsView {
    #[serde(default)]
    pub ebs_snapshot_preservation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrgConfigView {
    #[serde(default)]
    pub auto_enable: bool,
    #[serde(default)]
    pub auto_enable_organization_members: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredMalwareScanView {
    pub scan_id: String,
    pub detector_id: String,
    #[serde(default)]
    pub resource_arn: Option<String>,
    #[serde(default)]
    pub resource_type: Option<String>,
    pub scan_type: String,
    pub scan_status: String,
    pub scan_started_at: f64,
    #[serde(default)]
    pub scan_completed_at: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishingDestinationView {
    pub destination_id: String,
    pub destination_type: String,
    pub status: String,
    #[serde(default)]
    pub destination_arn: Option<String>,
    #[serde(default)]
    pub kms_key_arn: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MalwareProtectionPlanView {
    pub plan_id: String,
    pub arn: String,
    pub role: String,
    pub protected_resource: MalwareProtectionPlanResourceView,
    #[serde(default)]
    pub actions: Option<MalwareProtectionPlanActionsView>,
    pub status: String,
    pub created_at: f64,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MalwareProtectionPlanResourceView {
    #[serde(default)]
    pub s3_bucket_name: Option<String>,
    #[serde(default)]
    pub s3_object_prefixes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MalwareProtectionPlanActionsView {
    #[serde(default)]
    pub tagging_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvitationView {
    pub account_id: String,
    pub invitation_id: String,
    pub invited_at: String,
    pub relationship_status: String,
}

// ---------------------------------------------------------------------------
// From conversions
// ---------------------------------------------------------------------------

impl From<&Condition> for ConditionView {
    fn from(c: &Condition) -> Self {
        ConditionView {
            eq: c.eq.clone(),
            neq: c.neq.clone(),
            gt: c.gt,
            gte: c.gte,
            lt: c.lt,
            lte: c.lte,
        }
    }
}

impl From<ConditionView> for Condition {
    fn from(v: ConditionView) -> Self {
        Condition {
            eq: v.eq,
            neq: v.neq,
            gt: v.gt,
            gte: v.gte,
            lt: v.lt,
            lte: v.lte,
        }
    }
}

impl From<&FindingCriteria> for FindingCriteriaView {
    fn from(fc: &FindingCriteria) -> Self {
        FindingCriteriaView {
            criterion: fc
                .criterion
                .iter()
                .map(|(k, v)| (k.clone(), ConditionView::from(v)))
                .collect(),
        }
    }
}

impl From<FindingCriteriaView> for FindingCriteria {
    fn from(v: FindingCriteriaView) -> Self {
        FindingCriteria {
            criterion: v
                .criterion
                .into_iter()
                .map(|(k, c)| (k, Condition::from(c)))
                .collect(),
        }
    }
}

impl From<&Filter> for FilterView {
    fn from(f: &Filter) -> Self {
        FilterView {
            name: f.name.clone(),
            description: f.description.clone(),
            action: f.action.clone(),
            rank: f.rank,
            finding_criteria: FindingCriteriaView::from(&f.finding_criteria),
            tags: f.tags.clone(),
        }
    }
}

impl From<FilterView> for Filter {
    fn from(v: FilterView) -> Self {
        Filter {
            name: v.name,
            description: v.description,
            action: v.action,
            rank: v.rank,
            finding_criteria: FindingCriteria::from(v.finding_criteria),
            tags: v.tags,
        }
    }
}

impl From<&DataSourcesConfig> for DataSourcesConfigView {
    fn from(ds: &DataSourcesConfig) -> Self {
        DataSourcesConfigView {
            s3_logs_enabled: ds.s3_logs_enabled,
            kubernetes_audit_logs_enabled: ds.kubernetes_audit_logs_enabled,
        }
    }
}

impl From<DataSourcesConfigView> for DataSourcesConfig {
    fn from(v: DataSourcesConfigView) -> Self {
        DataSourcesConfig {
            s3_logs_enabled: v.s3_logs_enabled,
            kubernetes_audit_logs_enabled: v.kubernetes_audit_logs_enabled,
        }
    }
}

impl From<&DetectorAdditionalConfiguration> for DetectorAdditionalConfigurationView {
    fn from(c: &DetectorAdditionalConfiguration) -> Self {
        DetectorAdditionalConfigurationView {
            name: c.name.clone(),
            status: c.status.clone(),
        }
    }
}

impl From<DetectorAdditionalConfigurationView> for DetectorAdditionalConfiguration {
    fn from(v: DetectorAdditionalConfigurationView) -> Self {
        DetectorAdditionalConfiguration {
            name: v.name,
            status: v.status,
        }
    }
}

impl From<&DetectorFeature> for DetectorFeatureView {
    fn from(f: &DetectorFeature) -> Self {
        DetectorFeatureView {
            name: f.name.clone(),
            status: f.status.clone(),
            additional_configuration: f
                .additional_configuration
                .iter()
                .map(DetectorAdditionalConfigurationView::from)
                .collect(),
        }
    }
}

impl From<DetectorFeatureView> for DetectorFeature {
    fn from(v: DetectorFeatureView) -> Self {
        DetectorFeature {
            name: v.name,
            status: v.status,
            additional_configuration: v
                .additional_configuration
                .into_iter()
                .map(DetectorAdditionalConfiguration::from)
                .collect(),
        }
    }
}

impl From<&StoredFinding> for StoredFindingView {
    fn from(f: &StoredFinding) -> Self {
        StoredFindingView {
            id: f.id.clone(),
            arn: f.arn.clone(),
            account_id: f.account_id.clone(),
            region: f.region.clone(),
            r#type: f.r#type.clone(),
            severity: f.severity,
            title: f.title.clone(),
            description: f.description.clone(),
            created_at: f.created_at.clone(),
            updated_at: f.updated_at.clone(),
            archived: f.archived,
        }
    }
}

impl From<StoredFindingView> for StoredFinding {
    fn from(v: StoredFindingView) -> Self {
        StoredFinding {
            id: v.id,
            arn: v.arn,
            account_id: v.account_id,
            region: v.region,
            r#type: v.r#type,
            severity: v.severity,
            title: v.title,
            description: v.description,
            created_at: v.created_at,
            updated_at: v.updated_at,
            archived: v.archived,
        }
    }
}

impl From<&Detector> for DetectorView {
    fn from(d: &Detector) -> Self {
        DetectorView {
            detector_id: d.detector_id.clone(),
            status: d.status.clone(),
            finding_publishing_frequency: d.finding_publishing_frequency.clone(),
            created_at: d.created_at.clone(),
            tags: d.tags.clone(),
            filters: d
                .filters
                .iter()
                .map(|(k, v)| (k.clone(), FilterView::from(v)))
                .collect(),
            data_sources: d.data_sources.as_ref().map(DataSourcesConfigView::from),
            features: d
                .features
                .as_ref()
                .map(|fs| fs.iter().map(DetectorFeatureView::from).collect()),
            ip_sets: d
                .ip_sets
                .iter()
                .map(|(k, v)| (k.clone(), IpSetView::from(v)))
                .collect(),
            threat_intel_sets: d
                .threat_intel_sets
                .iter()
                .map(|(k, v)| (k.clone(), ThreatIntelSetView::from(v)))
                .collect(),
            threat_entity_sets: d
                .threat_entity_sets
                .iter()
                .map(|(k, v)| (k.clone(), ThreatEntitySetView::from(v)))
                .collect(),
            trusted_entity_sets: d
                .trusted_entity_sets
                .iter()
                .map(|(k, v)| (k.clone(), TrustedEntitySetView::from(v)))
                .collect(),
            findings: d
                .findings
                .iter()
                .map(|(k, v)| (k.clone(), StoredFindingView::from(v)))
                .collect(),
            administrator_account_id: d.administrator_account_id.clone(),
            master_account_id: d.master_account_id.clone(),
            coverage_records: d
                .coverage_records
                .iter()
                .map(CoverageRecordView::from)
                .collect(),
            members: d
                .members
                .iter()
                .map(|(k, v)| (k.clone(), MemberView::from(v)))
                .collect(),
            malware_scan_settings: MalwareScanSettingsView::from(&d.malware_scan_settings),
            org_config: OrgConfigView::from(&d.org_config),
            malware_scans: d
                .malware_scans
                .iter()
                .map(|(k, v)| (k.clone(), StoredMalwareScanView::from(v)))
                .collect(),
        }
    }
}

impl From<DetectorView> for Detector {
    fn from(v: DetectorView) -> Self {
        Detector {
            detector_id: v.detector_id,
            status: v.status,
            finding_publishing_frequency: v.finding_publishing_frequency,
            created_at: v.created_at,
            tags: v.tags,
            filters: v
                .filters
                .into_iter()
                .map(|(k, f)| (k, Filter::from(f)))
                .collect(),
            data_sources: v.data_sources.map(DataSourcesConfig::from),
            features: v
                .features
                .map(|fs| fs.into_iter().map(DetectorFeature::from).collect()),
            ip_sets: v
                .ip_sets
                .into_iter()
                .map(|(k, s)| (k, IpSet::from(s)))
                .collect(),
            threat_intel_sets: v
                .threat_intel_sets
                .into_iter()
                .map(|(k, s)| (k, ThreatIntelSet::from(s)))
                .collect(),
            threat_entity_sets: v
                .threat_entity_sets
                .into_iter()
                .map(|(k, s)| (k, ThreatEntitySet::from(s)))
                .collect(),
            trusted_entity_sets: v
                .trusted_entity_sets
                .into_iter()
                .map(|(k, s)| (k, TrustedEntitySet::from(s)))
                .collect(),
            findings: v
                .findings
                .into_iter()
                .map(|(k, f)| (k, StoredFinding::from(f)))
                .collect(),
            members: v
                .members
                .into_iter()
                .map(|(k, m)| (k, Member::from(m)))
                .collect(),
            malware_scan_settings: MalwareScanSettings::from(v.malware_scan_settings),
            org_config: OrgConfig::from(v.org_config),
            malware_scans: v
                .malware_scans
                .into_iter()
                .map(|(k, s)| (k, StoredMalwareScan::from(s)))
                .collect(),
            administrator_account_id: v.administrator_account_id,
            master_account_id: v.master_account_id,
            coverage_records: v
                .coverage_records
                .into_iter()
                .map(CoverageRecord::from)
                .collect(),
        }
    }
}

impl From<&AdminAccount> for AdminAccountView {
    fn from(a: &AdminAccount) -> Self {
        AdminAccountView {
            admin_account_id: a.admin_account_id.clone(),
            admin_status: a.admin_status.clone(),
        }
    }
}

impl From<AdminAccountView> for AdminAccount {
    fn from(v: AdminAccountView) -> Self {
        AdminAccount {
            admin_account_id: v.admin_account_id,
            admin_status: v.admin_status,
        }
    }
}

impl From<&IpSet> for IpSetView {
    fn from(s: &IpSet) -> Self {
        IpSetView {
            ip_set_id: s.ip_set_id.clone(),
            name: s.name.clone(),
            format: s.format.clone(),
            location: s.location.clone(),
            status: s.status.clone(),
            tags: s.tags.clone(),
        }
    }
}

impl From<IpSetView> for IpSet {
    fn from(v: IpSetView) -> Self {
        IpSet {
            ip_set_id: v.ip_set_id,
            name: v.name,
            format: v.format,
            location: v.location,
            status: v.status,
            tags: v.tags,
        }
    }
}

impl From<&ThreatIntelSet> for ThreatIntelSetView {
    fn from(s: &ThreatIntelSet) -> Self {
        ThreatIntelSetView {
            threat_intel_set_id: s.threat_intel_set_id.clone(),
            name: s.name.clone(),
            format: s.format.clone(),
            location: s.location.clone(),
            status: s.status.clone(),
            tags: s.tags.clone(),
        }
    }
}

impl From<ThreatIntelSetView> for ThreatIntelSet {
    fn from(v: ThreatIntelSetView) -> Self {
        ThreatIntelSet {
            threat_intel_set_id: v.threat_intel_set_id,
            name: v.name,
            format: v.format,
            location: v.location,
            status: v.status,
            tags: v.tags,
        }
    }
}

impl From<&ThreatEntitySet> for ThreatEntitySetView {
    fn from(s: &ThreatEntitySet) -> Self {
        ThreatEntitySetView {
            threat_entity_set_id: s.threat_entity_set_id.clone(),
            name: s.name.clone(),
            format: s.format.clone(),
            location: s.location.clone(),
            status: s.status.clone(),
            tags: s.tags.clone(),
        }
    }
}

impl From<ThreatEntitySetView> for ThreatEntitySet {
    fn from(v: ThreatEntitySetView) -> Self {
        ThreatEntitySet {
            threat_entity_set_id: v.threat_entity_set_id,
            name: v.name,
            format: v.format,
            location: v.location,
            status: v.status,
            tags: v.tags,
        }
    }
}

impl From<&TrustedEntitySet> for TrustedEntitySetView {
    fn from(s: &TrustedEntitySet) -> Self {
        TrustedEntitySetView {
            trusted_entity_set_id: s.trusted_entity_set_id.clone(),
            name: s.name.clone(),
            format: s.format.clone(),
            location: s.location.clone(),
            status: s.status.clone(),
            tags: s.tags.clone(),
        }
    }
}

impl From<TrustedEntitySetView> for TrustedEntitySet {
    fn from(v: TrustedEntitySetView) -> Self {
        TrustedEntitySet {
            trusted_entity_set_id: v.trusted_entity_set_id,
            name: v.name,
            format: v.format,
            location: v.location,
            status: v.status,
            tags: v.tags,
        }
    }
}

impl From<&CoverageRecord> for CoverageRecordView {
    fn from(r: &CoverageRecord) -> Self {
        CoverageRecordView {
            account_id: r.account_id.clone(),
            detector_id: r.detector_id.clone(),
            coverage_status: r.coverage_status.clone(),
            resource_id: r.resource_id.clone(),
        }
    }
}

impl From<CoverageRecordView> for CoverageRecord {
    fn from(v: CoverageRecordView) -> Self {
        CoverageRecord {
            account_id: v.account_id,
            detector_id: v.detector_id,
            coverage_status: v.coverage_status,
            resource_id: v.resource_id,
        }
    }
}

impl From<&MalwareScanRecord> for MalwareScanRecordView {
    fn from(r: &MalwareScanRecord) -> Self {
        MalwareScanRecordView {
            scan_id: r.scan_id.clone(),
            status: r.status.clone(),
            created_at: r.created_at.clone(),
        }
    }
}

impl From<MalwareScanRecordView> for MalwareScanRecord {
    fn from(v: MalwareScanRecordView) -> Self {
        MalwareScanRecord {
            scan_id: v.scan_id,
            status: v.status,
            created_at: v.created_at,
        }
    }
}

impl From<&MemberDetectorAdditionalConfig> for MemberDetectorAdditionalConfigView {
    fn from(c: &MemberDetectorAdditionalConfig) -> Self {
        MemberDetectorAdditionalConfigView {
            name: c.name.clone(),
            status: c.status.clone(),
        }
    }
}

impl From<MemberDetectorAdditionalConfigView> for MemberDetectorAdditionalConfig {
    fn from(v: MemberDetectorAdditionalConfigView) -> Self {
        MemberDetectorAdditionalConfig {
            name: v.name,
            status: v.status,
        }
    }
}

impl From<&MemberDetectorFeature> for MemberDetectorFeatureView {
    fn from(f: &MemberDetectorFeature) -> Self {
        MemberDetectorFeatureView {
            name: f.name.clone(),
            status: f.status.clone(),
            additional_configuration: f
                .additional_configuration
                .iter()
                .map(MemberDetectorAdditionalConfigView::from)
                .collect(),
        }
    }
}

impl From<MemberDetectorFeatureView> for MemberDetectorFeature {
    fn from(v: MemberDetectorFeatureView) -> Self {
        MemberDetectorFeature {
            name: v.name,
            status: v.status,
            additional_configuration: v
                .additional_configuration
                .into_iter()
                .map(MemberDetectorAdditionalConfig::from)
                .collect(),
        }
    }
}

impl From<&Member> for MemberView {
    fn from(m: &Member) -> Self {
        MemberView {
            account_id: m.account_id.clone(),
            email: m.email.clone(),
            relationship_status: m.relationship_status.clone(),
            invited_at: m.invited_at.clone(),
            updated_at: m.updated_at.clone(),
            detector_features: m
                .detector_features
                .iter()
                .map(MemberDetectorFeatureView::from)
                .collect(),
        }
    }
}

impl From<MemberView> for Member {
    fn from(v: MemberView) -> Self {
        Member {
            account_id: v.account_id,
            email: v.email,
            relationship_status: v.relationship_status,
            invited_at: v.invited_at,
            updated_at: v.updated_at,
            detector_features: v
                .detector_features
                .into_iter()
                .map(MemberDetectorFeature::from)
                .collect(),
        }
    }
}

impl From<&MalwareScanSettings> for MalwareScanSettingsView {
    fn from(s: &MalwareScanSettings) -> Self {
        MalwareScanSettingsView {
            ebs_snapshot_preservation: s.ebs_snapshot_preservation.clone(),
        }
    }
}

impl From<MalwareScanSettingsView> for MalwareScanSettings {
    fn from(v: MalwareScanSettingsView) -> Self {
        MalwareScanSettings {
            ebs_snapshot_preservation: v.ebs_snapshot_preservation,
        }
    }
}

impl From<&OrgConfig> for OrgConfigView {
    fn from(c: &OrgConfig) -> Self {
        OrgConfigView {
            auto_enable: c.auto_enable,
            auto_enable_organization_members: c.auto_enable_organization_members.clone(),
        }
    }
}

impl From<OrgConfigView> for OrgConfig {
    fn from(v: OrgConfigView) -> Self {
        OrgConfig {
            auto_enable: v.auto_enable,
            auto_enable_organization_members: v.auto_enable_organization_members,
        }
    }
}

impl From<&StoredMalwareScan> for StoredMalwareScanView {
    fn from(s: &StoredMalwareScan) -> Self {
        StoredMalwareScanView {
            scan_id: s.scan_id.clone(),
            detector_id: s.detector_id.clone(),
            resource_arn: s.resource_arn.clone(),
            resource_type: s.resource_type.clone(),
            scan_type: s.scan_type.clone(),
            scan_status: s.scan_status.clone(),
            scan_started_at: s.scan_started_at,
            scan_completed_at: s.scan_completed_at,
        }
    }
}

impl From<StoredMalwareScanView> for StoredMalwareScan {
    fn from(v: StoredMalwareScanView) -> Self {
        StoredMalwareScan {
            scan_id: v.scan_id,
            detector_id: v.detector_id,
            resource_arn: v.resource_arn,
            resource_type: v.resource_type,
            scan_type: v.scan_type,
            scan_status: v.scan_status,
            scan_started_at: v.scan_started_at,
            scan_completed_at: v.scan_completed_at,
        }
    }
}

impl From<&PublishingDestination> for PublishingDestinationView {
    fn from(d: &PublishingDestination) -> Self {
        PublishingDestinationView {
            destination_id: d.destination_id.clone(),
            destination_type: d.destination_type.clone(),
            status: d.status.clone(),
            destination_arn: d.destination_arn.clone(),
            kms_key_arn: d.kms_key_arn.clone(),
            tags: d.tags.clone(),
        }
    }
}

impl From<PublishingDestinationView> for PublishingDestination {
    fn from(v: PublishingDestinationView) -> Self {
        PublishingDestination {
            destination_id: v.destination_id,
            destination_type: v.destination_type,
            status: v.status,
            destination_arn: v.destination_arn,
            kms_key_arn: v.kms_key_arn,
            tags: v.tags,
        }
    }
}

impl From<&MalwareProtectionPlanActions> for MalwareProtectionPlanActionsView {
    fn from(a: &MalwareProtectionPlanActions) -> Self {
        MalwareProtectionPlanActionsView {
            tagging_status: a.tagging_status.clone(),
        }
    }
}

impl From<MalwareProtectionPlanActionsView> for MalwareProtectionPlanActions {
    fn from(v: MalwareProtectionPlanActionsView) -> Self {
        MalwareProtectionPlanActions {
            tagging_status: v.tagging_status,
        }
    }
}

impl From<&MalwareProtectionPlanResource> for MalwareProtectionPlanResourceView {
    fn from(r: &MalwareProtectionPlanResource) -> Self {
        MalwareProtectionPlanResourceView {
            s3_bucket_name: r.s3_bucket_name.clone(),
            s3_object_prefixes: r.s3_object_prefixes.clone(),
        }
    }
}

impl From<MalwareProtectionPlanResourceView> for MalwareProtectionPlanResource {
    fn from(v: MalwareProtectionPlanResourceView) -> Self {
        MalwareProtectionPlanResource {
            s3_bucket_name: v.s3_bucket_name,
            s3_object_prefixes: v.s3_object_prefixes,
        }
    }
}

impl From<&MalwareProtectionPlan> for MalwareProtectionPlanView {
    fn from(p: &MalwareProtectionPlan) -> Self {
        MalwareProtectionPlanView {
            plan_id: p.plan_id.clone(),
            arn: p.arn.clone(),
            role: p.role.clone(),
            protected_resource: MalwareProtectionPlanResourceView::from(&p.protected_resource),
            actions: p
                .actions
                .as_ref()
                .map(MalwareProtectionPlanActionsView::from),
            status: p.status.clone(),
            created_at: p.created_at,
            tags: p.tags.clone(),
        }
    }
}

impl From<MalwareProtectionPlanView> for MalwareProtectionPlan {
    fn from(v: MalwareProtectionPlanView) -> Self {
        MalwareProtectionPlan {
            plan_id: v.plan_id,
            arn: v.arn,
            role: v.role,
            protected_resource: MalwareProtectionPlanResource::from(v.protected_resource),
            actions: v.actions.map(MalwareProtectionPlanActions::from),
            status: v.status,
            created_at: v.created_at,
            tags: v.tags,
        }
    }
}

impl From<&Invitation> for InvitationView {
    fn from(i: &Invitation) -> Self {
        InvitationView {
            account_id: i.account_id.clone(),
            invitation_id: i.invitation_id.clone(),
            invited_at: i.invited_at.clone(),
            relationship_status: i.relationship_status.clone(),
        }
    }
}

impl From<InvitationView> for Invitation {
    fn from(v: InvitationView) -> Self {
        Invitation {
            account_id: v.account_id,
            invitation_id: v.invitation_id,
            invited_at: v.invited_at,
            relationship_status: v.relationship_status,
        }
    }
}

// ---------------------------------------------------------------------------
// StatefulService implementation
// ---------------------------------------------------------------------------

impl StatefulService for GuardDutyService {
    type StateView = GuardDutyStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;

        let detectors = guard
            .detectors
            .iter()
            .map(|(k, v)| (k.clone(), DetectorView::from(v)))
            .collect();

        let admin_accounts = guard
            .admin_accounts
            .iter()
            .map(AdminAccountView::from)
            .collect();

        let resource_tags = guard.resource_tags.clone();

        let malware_scans = guard
            .malware_scans
            .iter()
            .map(|(k, v)| (k.clone(), MalwareScanRecordView::from(v)))
            .collect();

        let publishing_destinations = guard
            .publishing_destinations
            .iter()
            .map(|(det_id, dests)| {
                (
                    det_id.clone(),
                    dests
                        .iter()
                        .map(|(dest_id, d)| (dest_id.clone(), PublishingDestinationView::from(d)))
                        .collect(),
                )
            })
            .collect();

        let malware_protection_plans = guard
            .malware_protection_plans
            .iter()
            .map(|(k, v)| (k.clone(), MalwareProtectionPlanView::from(v)))
            .collect();

        let invitations = guard
            .invitations
            .iter()
            .map(|(k, v)| (k.clone(), InvitationView::from(v)))
            .collect();

        GuardDutyStateView {
            detectors,
            admin_accounts,
            resource_tags,
            malware_scans,
            publishing_destinations,
            malware_protection_plans,
            invitations,
        }
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let mut new_state = GuardDutyState::default();
        for (id, dv) in view.detectors {
            new_state.detectors.insert(id, Detector::from(dv));
        }
        new_state.admin_accounts = view
            .admin_accounts
            .into_iter()
            .map(AdminAccount::from)
            .collect();
        new_state.resource_tags = view.resource_tags;
        new_state.malware_scans = view
            .malware_scans
            .into_iter()
            .map(|(k, v)| (k, MalwareScanRecord::from(v)))
            .collect();
        new_state.publishing_destinations = view
            .publishing_destinations
            .into_iter()
            .map(|(det_id, dests)| {
                (
                    det_id,
                    dests
                        .into_iter()
                        .map(|(dest_id, d)| (dest_id, PublishingDestination::from(d)))
                        .collect(),
                )
            })
            .collect();
        new_state.malware_protection_plans = view
            .malware_protection_plans
            .into_iter()
            .map(|(k, v)| (k, MalwareProtectionPlan::from(v)))
            .collect();
        new_state.invitations = view
            .invitations
            .into_iter()
            .map(|(k, v)| (k, Invitation::from(v)))
            .collect();
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
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (id, dv) in view.detectors {
                guard.detectors.insert(id, Detector::from(dv));
            }
            for av in view.admin_accounts {
                let exists = guard
                    .admin_accounts
                    .iter()
                    .any(|a| a.admin_account_id == av.admin_account_id);
                if !exists {
                    guard.admin_accounts.push(AdminAccount::from(av));
                }
            }
            for (arn, tags) in view.resource_tags {
                let entry = guard.resource_tags.entry(arn).or_default();
                for (k, v) in tags {
                    entry.insert(k, v);
                }
            }
            for (id, sv) in view.malware_scans {
                guard
                    .malware_scans
                    .entry(id)
                    .or_insert_with(|| MalwareScanRecord::from(sv));
            }
            for (det_id, dests) in view.publishing_destinations {
                let entry = guard.publishing_destinations.entry(det_id).or_default();
                for (dest_id, dv) in dests {
                    entry
                        .entry(dest_id)
                        .or_insert_with(|| PublishingDestination::from(dv));
                }
            }
            for (plan_id, pv) in view.malware_protection_plans {
                guard
                    .malware_protection_plans
                    .entry(plan_id)
                    .or_insert_with(|| MalwareProtectionPlan::from(pv));
            }
            for (acct_id, iv) in view.invitations {
                guard
                    .invitations
                    .entry(acct_id)
                    .or_insert_with(|| Invitation::from(iv));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

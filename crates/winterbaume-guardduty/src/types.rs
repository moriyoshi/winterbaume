use std::collections::HashMap;

/// A GuardDuty detector.
#[derive(Debug, Clone)]
pub struct Detector {
    pub detector_id: String,
    pub status: String,
    pub finding_publishing_frequency: String,
    pub created_at: String,
    pub tags: HashMap<String, String>,
    pub filters: HashMap<String, Filter>,
    pub data_sources: Option<DataSourcesConfig>,
    pub features: Option<Vec<DetectorFeature>>,
    pub ip_sets: HashMap<String, IpSet>,
    pub threat_intel_sets: HashMap<String, ThreatIntelSet>,
    pub threat_entity_sets: HashMap<String, ThreatEntitySet>,
    pub trusted_entity_sets: HashMap<String, TrustedEntitySet>,
    /// Findings keyed by finding ID.
    pub findings: HashMap<String, StoredFinding>,
    /// Members keyed by account ID (for the administrator detector).
    pub members: HashMap<String, Member>,
    /// Malware scan settings for this detector.
    pub malware_scan_settings: MalwareScanSettings,
    /// Organization configuration for this detector.
    pub org_config: OrgConfig,
    /// Malware scans keyed by scan ID.
    pub malware_scans: HashMap<String, StoredMalwareScan>,
    /// The administrator account ID associated with this detector (set via accept invitation).
    pub administrator_account_id: Option<String>,
    /// The master account ID associated with this detector (legacy, set via accept invitation).
    pub master_account_id: Option<String>,
    /// Coverage records for this detector.
    pub coverage_records: Vec<CoverageRecord>,
}

/// A stored GuardDuty finding.
#[derive(Debug, Clone)]
pub struct StoredFinding {
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

/// Data sources configuration stored on a detector.
#[derive(Debug, Clone, Default)]
pub struct DataSourcesConfig {
    pub s3_logs_enabled: Option<bool>,
    pub kubernetes_audit_logs_enabled: Option<bool>,
}

/// A detector feature.
#[derive(Debug, Clone)]
pub struct DetectorFeature {
    pub name: String,
    pub status: String,
    pub additional_configuration: Vec<DetectorAdditionalConfiguration>,
}

/// Additional configuration for a detector feature.
#[derive(Debug, Clone)]
pub struct DetectorAdditionalConfiguration {
    pub name: String,
    pub status: String,
}

/// A GuardDuty filter.
#[derive(Debug, Clone)]
pub struct Filter {
    pub name: String,
    pub description: String,
    pub action: String,
    pub rank: i32,
    pub finding_criteria: FindingCriteria,
    pub tags: HashMap<String, String>,
}

/// Finding criteria used by filters.
#[derive(Debug, Clone, Default)]
pub struct FindingCriteria {
    pub criterion: HashMap<String, Condition>,
}

/// A condition within finding criteria.
#[derive(Debug, Clone, Default)]
pub struct Condition {
    pub eq: Option<Vec<String>>,
    pub neq: Option<Vec<String>>,
    pub gt: Option<i64>,
    pub gte: Option<i64>,
    pub lt: Option<i64>,
    pub lte: Option<i64>,
}

/// An organization admin account.
#[derive(Debug, Clone)]
pub struct AdminAccount {
    pub admin_account_id: String,
    pub admin_status: String,
}

/// A GuardDuty IP set.
#[derive(Debug, Clone)]
pub struct IpSet {
    pub ip_set_id: String,
    pub name: String,
    pub format: String,
    pub location: String,
    pub status: String,
    pub tags: HashMap<String, String>,
}

/// A GuardDuty threat intel set.
#[derive(Debug, Clone)]
pub struct ThreatIntelSet {
    pub threat_intel_set_id: String,
    pub name: String,
    pub format: String,
    pub location: String,
    pub status: String,
    pub tags: HashMap<String, String>,
}

/// A GuardDuty threat entity set.
#[derive(Debug, Clone)]
pub struct ThreatEntitySet {
    pub threat_entity_set_id: String,
    pub name: String,
    pub format: String,
    pub location: String,
    pub status: String,
    pub tags: HashMap<String, String>,
}

/// A GuardDuty publishing destination.
#[derive(Debug, Clone)]
pub struct PublishingDestination {
    pub destination_id: String,
    pub destination_type: String,
    pub status: String,
    pub destination_arn: Option<String>,
    pub kms_key_arn: Option<String>,
    pub tags: std::collections::HashMap<String, String>,
}

/// A GuardDuty trusted entity set.
#[derive(Debug, Clone)]
pub struct TrustedEntitySet {
    pub trusted_entity_set_id: String,
    pub name: String,
    pub format: String,
    pub location: String,
    pub status: String,
    pub tags: HashMap<String, String>,
}

/// Per-member detector feature configuration stored on the administrator detector.
#[derive(Debug, Clone)]
pub struct MemberDetectorFeature {
    pub name: String,
    pub status: String,
    pub additional_configuration: Vec<MemberDetectorAdditionalConfig>,
}

/// Additional configuration entry for a member detector feature.
#[derive(Debug, Clone)]
pub struct MemberDetectorAdditionalConfig {
    pub name: String,
    pub status: String,
}

/// A GuardDuty member account.
#[derive(Debug, Clone)]
pub struct Member {
    pub account_id: String,
    pub email: String,
    pub relationship_status: String,
    pub invited_at: Option<String>,
    pub updated_at: String,
    /// Per-member detector feature overrides set via UpdateMemberDetectors.
    pub detector_features: Vec<MemberDetectorFeature>,
}

/// A pending invitation sent to a member account.
#[derive(Debug, Clone)]
pub struct Invitation {
    pub account_id: String,
    pub invitation_id: String,
    pub invited_at: String,
    pub relationship_status: String,
}

/// A malware protection plan.
#[derive(Debug, Clone)]
pub struct MalwareProtectionPlan {
    pub plan_id: String,
    pub arn: String,
    pub role: String,
    pub protected_resource: MalwareProtectionPlanResource,
    pub actions: Option<MalwareProtectionPlanActions>,
    pub status: String,
    pub created_at: f64,
    pub tags: HashMap<String, String>,
}

/// Protected resource for a malware protection plan.
#[derive(Debug, Clone, Default)]
pub struct MalwareProtectionPlanResource {
    pub s3_bucket_name: Option<String>,
    pub s3_object_prefixes: Vec<String>,
}

/// Actions configuration for a malware protection plan.
#[derive(Debug, Clone)]
pub struct MalwareProtectionPlanActions {
    pub tagging_status: Option<String>,
}

/// Malware scan settings stored per detector.
#[derive(Debug, Clone, Default)]
pub struct MalwareScanSettings {
    pub ebs_snapshot_preservation: Option<String>,
}

/// Organization configuration stored per detector (for the delegated admin).
#[derive(Debug, Clone, Default)]
pub struct OrgConfig {
    pub auto_enable: bool,
    pub auto_enable_organization_members: Option<String>,
}

/// A stored malware scan record.
#[derive(Debug, Clone)]
pub struct StoredMalwareScan {
    pub scan_id: String,
    pub detector_id: String,
    pub resource_arn: Option<String>,
    pub resource_type: Option<String>,
    pub scan_type: String,
    pub scan_status: String,
    pub scan_started_at: f64,
    pub scan_completed_at: Option<f64>,
}

/// A coverage record for a GuardDuty detector.
#[derive(Debug, Clone)]
pub struct CoverageRecord {
    pub account_id: String,
    pub detector_id: String,
    pub coverage_status: String,
    pub resource_id: String,
}

/// A malware scan record for S3 objects.
#[derive(Debug, Clone)]
pub struct MalwareScanRecord {
    pub scan_id: String,
    pub status: String,
    pub created_at: String,
}

use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// An ECR repository.
#[derive(Debug, Clone)]
pub struct Repository {
    pub repository_name: String,
    pub repository_arn: String,
    pub repository_uri: String,
    pub registry_id: String,
    pub created_at: DateTime<Utc>,
    pub images: Vec<Image>,
    pub tags: HashMap<String, String>,
    pub lifecycle_policy: Option<String>,
    pub repository_policy: Option<String>,
    pub image_scanning_configuration: ImageScanningConfiguration,
    pub image_tag_mutability: String,
    pub encryption_configuration: EncryptionConfiguration,
}

/// An ECR image.
#[derive(Debug, Clone)]
pub struct Image {
    pub image_digest: String,
    pub image_tags: Vec<String>,
    pub image_manifest: String,
    pub pushed_at: DateTime<Utc>,
    pub image_scan_status: Option<ImageScanStatusData>,
    pub image_scan_findings: Option<ImageScanFindingsData>,
}

/// Image scanning configuration for a repository.
#[derive(Debug, Clone, Default)]
pub struct ImageScanningConfiguration {
    pub scan_on_push: bool,
}

/// Encryption configuration for a repository.
#[derive(Debug, Clone)]
pub struct EncryptionConfiguration {
    pub encryption_type: String,
    pub kms_key: Option<String>,
}

impl Default for EncryptionConfiguration {
    fn default() -> Self {
        Self {
            encryption_type: "AES256".to_string(),
            kms_key: None,
        }
    }
}

/// Image scan status.
#[derive(Debug, Clone)]
pub struct ImageScanStatusData {
    pub status: String,
    pub description: String,
}

/// Image scan findings.
#[derive(Debug, Clone)]
pub struct ImageScanFindingsData {
    pub image_scan_completed_at: DateTime<Utc>,
    pub vulnerability_source_updated_at: DateTime<Utc>,
    pub finding_severity_counts: HashMap<String, i32>,
}

/// Registry-level scanning configuration.
#[derive(Debug, Clone)]
pub struct RegistryScanningConfig {
    pub scan_type: String,
    pub rules: Vec<RegistryScanningRuleData>,
}

impl Default for RegistryScanningConfig {
    fn default() -> Self {
        Self {
            scan_type: "BASIC".to_string(),
            rules: Vec::new(),
        }
    }
}

/// A scanning rule.
#[derive(Debug, Clone)]
pub struct RegistryScanningRuleData {
    pub scan_frequency: String,
    pub repository_filters: Vec<ScanningRepositoryFilterData>,
}

/// A scanning repository filter.
#[derive(Debug, Clone)]
pub struct ScanningRepositoryFilterData {
    pub filter: String,
    pub filter_type: String,
}

/// Replication configuration.
#[derive(Debug, Clone, Default)]
pub struct ReplicationConfig {
    pub rules: Vec<ReplicationRuleData>,
}

/// A replication rule.
#[derive(Debug, Clone)]
pub struct ReplicationRuleData {
    pub destinations: Vec<ReplicationDestinationData>,
    pub repository_filters: Vec<RepositoryFilterData>,
}

/// A replication destination.
#[derive(Debug, Clone)]
pub struct ReplicationDestinationData {
    pub region: String,
    pub registry_id: String,
}

/// A repository filter for replication.
#[derive(Debug, Clone)]
pub struct RepositoryFilterData {
    pub filter: String,
    pub filter_type: String,
}

/// An in-progress layer upload.
#[derive(Debug, Clone)]
pub struct LayerUpload {
    pub upload_id: String,
    pub registry_id: String,
    pub repository_name: String,
    pub last_byte_received: i64,
}

/// A pull-through cache rule.
#[derive(Debug, Clone)]
pub struct PullThroughCacheRuleData {
    pub ecr_repository_prefix: String,
    pub upstream_registry_url: String,
    pub created_at: chrono::DateTime<Utc>,
    pub registry_id: String,
    pub upstream_registry: Option<String>,
    pub upstream_repository_prefix: Option<String>,
    pub credential_arn: Option<String>,
    pub custom_role_arn: Option<String>,
}

/// A repository creation template.
#[derive(Debug, Clone)]
pub struct RepositoryCreationTemplateData {
    pub prefix: String,
    pub description: Option<String>,
    pub lifecycle_policy: Option<String>,
    pub repository_policy: Option<String>,
    pub image_tag_mutability: Option<String>,
    pub custom_role_arn: Option<String>,
    pub applied_for: Vec<String>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

/// Signing configuration rules.
#[derive(Debug, Clone, Default)]
pub struct SigningConfigData {
    pub rules: Vec<SigningRuleData>,
}

#[derive(Debug, Clone)]
pub struct SigningRuleData {
    pub signing_profile_arn: String,
    pub repository_filters: Vec<SigningRepositoryFilterData>,
}

#[derive(Debug, Clone)]
pub struct SigningRepositoryFilterData {
    pub filter: String,
    pub filter_type: String,
}

/// A pull-time update exclusion entry.
#[derive(Debug, Clone)]
pub struct PullTimeUpdateExclusionData {
    pub principal_arn: String,
    pub created_at: chrono::DateTime<Utc>,
}

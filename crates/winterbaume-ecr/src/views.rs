//! Serde-compatible view types for ECR state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::EcrService;
use crate::state::EcrState;
use crate::types::{
    EncryptionConfiguration, Image, ImageScanFindingsData, ImageScanStatusData,
    ImageScanningConfiguration, LayerUpload, PullThroughCacheRuleData, RegistryScanningConfig,
    RegistryScanningRuleData, ReplicationConfig, ReplicationDestinationData, ReplicationRuleData,
    Repository, RepositoryCreationTemplateData, RepositoryFilterData, ScanningRepositoryFilterData,
    SigningConfigData, SigningRepositoryFilterData, SigningRuleData,
};

/// Serializable view of the entire ECR state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EcrStateView {
    /// Repositories keyed by repository name.
    #[serde(default)]
    pub repositories: HashMap<String, RepositoryView>,
    /// Registry-level policy text.
    pub registry_policy: Option<String>,
    /// Registry-level scanning configuration.
    #[serde(default)]
    pub registry_scanning_configuration: RegistryScanningConfigView,
    /// Registry-level replication configuration.
    #[serde(default)]
    pub replication_configuration: ReplicationConfigView,
    /// In-progress layer uploads keyed by upload_id.
    #[serde(default)]
    pub layer_uploads: HashMap<String, LayerUploadView>,
    /// Pull-through cache rules keyed by ecr_repository_prefix.
    #[serde(default)]
    pub pull_through_cache_rules: Vec<PullThroughCacheRuleView>,
    /// Repository creation templates keyed by prefix.
    #[serde(default)]
    pub repository_creation_templates: Vec<RepositoryCreationTemplateView>,
    /// Registry-level signing configuration.
    #[serde(default)]
    pub signing_config: SigningConfigView,
    /// Pull-time update exclusions keyed by principal_arn.
    #[serde(default)]
    pub pull_time_update_exclusions: Vec<PullTimeUpdateExclusionView>,
    /// Account settings keyed by name.
    #[serde(default)]
    pub account_settings: HashMap<String, String>,
}

/// Serializable view of a single ECR repository.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryView {
    pub repository_name: String,
    pub repository_arn: String,
    pub repository_uri: String,
    pub registry_id: String,
    /// Creation timestamp in RFC 3339 format.
    pub created_at: Option<String>,
    /// Images stored in this repository.
    #[serde(default)]
    pub images: Vec<ImageView>,
    /// Resource tags.
    #[serde(default)]
    pub tags: HashMap<String, String>,
    /// Raw lifecycle policy text (if set).
    pub lifecycle_policy: Option<String>,
    /// Raw repository policy text (if set).
    pub repository_policy: Option<String>,
    pub image_scanning_configuration: ImageScanningConfigView,
    pub image_tag_mutability: String,
    #[serde(default)]
    pub encryption_configuration: EncryptionConfigView,
}

/// Serialisable view of an ECR encryption configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfigView {
    pub encryption_type: String,
    pub kms_key: Option<String>,
}

impl Default for EncryptionConfigView {
    fn default() -> Self {
        Self {
            encryption_type: "AES256".to_string(),
            kms_key: None,
        }
    }
}

/// Serializable view of an ECR image.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageView {
    pub image_digest: String,
    #[serde(default)]
    pub image_tags: Vec<String>,
    pub image_manifest: String,
    /// Push timestamp in RFC 3339 format.
    pub pushed_at: Option<String>,
    pub image_scan_status: Option<ImageScanStatusView>,
    pub image_scan_findings: Option<ImageScanFindingsView>,
}

/// Serializable view of image scanning configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImageScanningConfigView {
    pub scan_on_push: bool,
}

/// Serializable view of image scan status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageScanStatusView {
    pub status: String,
    pub description: String,
}

/// Serializable view of image scan findings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageScanFindingsView {
    /// Timestamp in RFC 3339 format.
    pub image_scan_completed_at: Option<String>,
    /// Timestamp in RFC 3339 format.
    pub vulnerability_source_updated_at: Option<String>,
    #[serde(default)]
    pub finding_severity_counts: HashMap<String, i32>,
}

/// Serializable view of registry-level scanning configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryScanningConfigView {
    pub scan_type: String,
    #[serde(default)]
    pub rules: Vec<RegistryScanningRuleView>,
}

impl Default for RegistryScanningConfigView {
    fn default() -> Self {
        Self {
            scan_type: "BASIC".to_string(),
            rules: Vec::new(),
        }
    }
}

/// Serializable view of a scanning rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryScanningRuleView {
    pub scan_frequency: String,
    #[serde(default)]
    pub repository_filters: Vec<ScanningRepositoryFilterView>,
}

/// Serializable view of a scanning repository filter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanningRepositoryFilterView {
    pub filter: String,
    pub filter_type: String,
}

/// Serializable view of replication configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReplicationConfigView {
    #[serde(default)]
    pub rules: Vec<ReplicationRuleView>,
}

/// Serializable view of a replication rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationRuleView {
    #[serde(default)]
    pub destinations: Vec<ReplicationDestinationView>,
    #[serde(default)]
    pub repository_filters: Vec<RepositoryFilterView>,
}

/// Serializable view of a replication destination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationDestinationView {
    pub region: String,
    pub registry_id: String,
}

/// Serializable view of a repository filter for replication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryFilterView {
    pub filter: String,
    pub filter_type: String,
}

/// Serializable view of an in-progress layer upload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerUploadView {
    pub upload_id: String,
    pub registry_id: String,
    pub repository_name: String,
    pub last_byte_received: i64,
}

/// Serializable view of a pull-through cache rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullThroughCacheRuleView {
    pub ecr_repository_prefix: String,
    pub upstream_registry_url: String,
    pub registry_id: Option<String>,
    pub created_at: Option<String>,
    pub upstream_registry: Option<String>,
    pub upstream_repository_prefix: Option<String>,
    pub credential_arn: Option<String>,
    pub custom_role_arn: Option<String>,
}

/// Serializable view of a repository creation template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryCreationTemplateView {
    pub prefix: String,
    pub description: Option<String>,
    pub lifecycle_policy: Option<String>,
    pub repository_policy: Option<String>,
    pub image_tag_mutability: Option<String>,
    pub custom_role_arn: Option<String>,
    #[serde(default)]
    pub applied_for: Vec<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

/// Serializable view of registry signing configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SigningConfigView {
    #[serde(default)]
    pub rules: Vec<SigningRuleView>,
}

/// Serializable view of a signing rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigningRuleView {
    pub signing_profile_arn: String,
    #[serde(default)]
    pub repository_filters: Vec<SigningRepositoryFilterView>,
}

/// Serializable view of a signing repository filter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigningRepositoryFilterView {
    pub filter: String,
    pub filter_type: String,
}

/// Serializable view of a pull-time update exclusion entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullTimeUpdateExclusionView {
    pub principal_arn: String,
    pub created_at: Option<String>,
}

// --- From internal types to view types ---

impl From<&EcrState> for EcrStateView {
    fn from(state: &EcrState) -> Self {
        EcrStateView {
            repositories: state
                .repositories
                .iter()
                .map(|(k, v)| (k.clone(), RepositoryView::from(v)))
                .collect(),
            registry_policy: state.registry_policy.clone(),
            registry_scanning_configuration: RegistryScanningConfigView::from(
                &state.registry_scanning_configuration,
            ),
            replication_configuration: ReplicationConfigView::from(
                &state.replication_configuration,
            ),
            layer_uploads: state
                .layer_uploads
                .iter()
                .map(|(k, v)| (k.clone(), LayerUploadView::from(v)))
                .collect(),
            pull_through_cache_rules: state
                .pull_through_cache_rules
                .values()
                .map(PullThroughCacheRuleView::from)
                .collect(),
            repository_creation_templates: state
                .repository_creation_templates
                .values()
                .map(RepositoryCreationTemplateView::from)
                .collect(),
            signing_config: SigningConfigView::from(&state.signing_config),
            pull_time_update_exclusions: state
                .pull_time_update_exclusions
                .values()
                .map(PullTimeUpdateExclusionView::from)
                .collect(),
            account_settings: state.account_settings.clone(),
        }
    }
}

impl From<&Repository> for RepositoryView {
    fn from(repo: &Repository) -> Self {
        RepositoryView {
            repository_name: repo.repository_name.clone(),
            repository_arn: repo.repository_arn.clone(),
            repository_uri: repo.repository_uri.clone(),
            registry_id: repo.registry_id.clone(),
            created_at: Some(repo.created_at.to_rfc3339()),
            images: repo.images.iter().map(ImageView::from).collect(),
            tags: repo.tags.clone(),
            lifecycle_policy: repo.lifecycle_policy.clone(),
            repository_policy: repo.repository_policy.clone(),
            image_scanning_configuration: ImageScanningConfigView::from(
                &repo.image_scanning_configuration,
            ),
            image_tag_mutability: repo.image_tag_mutability.clone(),
            encryption_configuration: EncryptionConfigView {
                encryption_type: repo.encryption_configuration.encryption_type.clone(),
                kms_key: repo.encryption_configuration.kms_key.clone(),
            },
        }
    }
}

impl From<&Image> for ImageView {
    fn from(img: &Image) -> Self {
        ImageView {
            image_digest: img.image_digest.clone(),
            image_tags: img.image_tags.clone(),
            image_manifest: img.image_manifest.clone(),
            pushed_at: Some(img.pushed_at.to_rfc3339()),
            image_scan_status: img
                .image_scan_status
                .as_ref()
                .map(ImageScanStatusView::from),
            image_scan_findings: img
                .image_scan_findings
                .as_ref()
                .map(ImageScanFindingsView::from),
        }
    }
}

impl From<&ImageScanningConfiguration> for ImageScanningConfigView {
    fn from(cfg: &ImageScanningConfiguration) -> Self {
        ImageScanningConfigView {
            scan_on_push: cfg.scan_on_push,
        }
    }
}

impl From<&ImageScanStatusData> for ImageScanStatusView {
    fn from(s: &ImageScanStatusData) -> Self {
        ImageScanStatusView {
            status: s.status.clone(),
            description: s.description.clone(),
        }
    }
}

impl From<&ImageScanFindingsData> for ImageScanFindingsView {
    fn from(f: &ImageScanFindingsData) -> Self {
        ImageScanFindingsView {
            image_scan_completed_at: Some(f.image_scan_completed_at.to_rfc3339()),
            vulnerability_source_updated_at: Some(f.vulnerability_source_updated_at.to_rfc3339()),
            finding_severity_counts: f.finding_severity_counts.clone(),
        }
    }
}

impl From<&RegistryScanningConfig> for RegistryScanningConfigView {
    fn from(cfg: &RegistryScanningConfig) -> Self {
        RegistryScanningConfigView {
            scan_type: cfg.scan_type.clone(),
            rules: cfg
                .rules
                .iter()
                .map(RegistryScanningRuleView::from)
                .collect(),
        }
    }
}

impl From<&RegistryScanningRuleData> for RegistryScanningRuleView {
    fn from(rule: &RegistryScanningRuleData) -> Self {
        RegistryScanningRuleView {
            scan_frequency: rule.scan_frequency.clone(),
            repository_filters: rule
                .repository_filters
                .iter()
                .map(ScanningRepositoryFilterView::from)
                .collect(),
        }
    }
}

impl From<&ScanningRepositoryFilterData> for ScanningRepositoryFilterView {
    fn from(f: &ScanningRepositoryFilterData) -> Self {
        ScanningRepositoryFilterView {
            filter: f.filter.clone(),
            filter_type: f.filter_type.clone(),
        }
    }
}

impl From<&ReplicationConfig> for ReplicationConfigView {
    fn from(cfg: &ReplicationConfig) -> Self {
        ReplicationConfigView {
            rules: cfg.rules.iter().map(ReplicationRuleView::from).collect(),
        }
    }
}

impl From<&ReplicationRuleData> for ReplicationRuleView {
    fn from(rule: &ReplicationRuleData) -> Self {
        ReplicationRuleView {
            destinations: rule
                .destinations
                .iter()
                .map(ReplicationDestinationView::from)
                .collect(),
            repository_filters: rule
                .repository_filters
                .iter()
                .map(RepositoryFilterView::from)
                .collect(),
        }
    }
}

impl From<&ReplicationDestinationData> for ReplicationDestinationView {
    fn from(d: &ReplicationDestinationData) -> Self {
        ReplicationDestinationView {
            region: d.region.clone(),
            registry_id: d.registry_id.clone(),
        }
    }
}

impl From<&RepositoryFilterData> for RepositoryFilterView {
    fn from(f: &RepositoryFilterData) -> Self {
        RepositoryFilterView {
            filter: f.filter.clone(),
            filter_type: f.filter_type.clone(),
        }
    }
}

impl From<&LayerUpload> for LayerUploadView {
    fn from(u: &LayerUpload) -> Self {
        LayerUploadView {
            upload_id: u.upload_id.clone(),
            registry_id: u.registry_id.clone(),
            repository_name: u.repository_name.clone(),
            last_byte_received: u.last_byte_received,
        }
    }
}

impl From<LayerUploadView> for LayerUpload {
    fn from(view: LayerUploadView) -> Self {
        LayerUpload {
            upload_id: view.upload_id,
            registry_id: view.registry_id,
            repository_name: view.repository_name,
            last_byte_received: view.last_byte_received,
        }
    }
}

// --- From view types to internal types ---

impl From<EcrStateView> for EcrState {
    fn from(view: EcrStateView) -> Self {
        EcrState {
            repositories: view
                .repositories
                .into_iter()
                .map(|(k, v)| (k, Repository::from(v)))
                .collect(),
            registry_policy: view.registry_policy,
            registry_scanning_configuration: RegistryScanningConfig::from(
                view.registry_scanning_configuration,
            ),
            replication_configuration: ReplicationConfig::from(view.replication_configuration),
            layer_uploads: view
                .layer_uploads
                .into_iter()
                .map(|(k, v)| (k, LayerUpload::from(v)))
                .collect(),
            pull_through_cache_rules: view
                .pull_through_cache_rules
                .into_iter()
                .map(|v| {
                    (
                        v.ecr_repository_prefix.clone(),
                        PullThroughCacheRuleData::from(v),
                    )
                })
                .collect(),
            repository_creation_templates: view
                .repository_creation_templates
                .into_iter()
                .map(|v| (v.prefix.clone(), RepositoryCreationTemplateData::from(v)))
                .collect(),
            signing_config: SigningConfigData::from(view.signing_config),
            pull_time_update_exclusions: view
                .pull_time_update_exclusions
                .into_iter()
                .map(|v| {
                    (
                        v.principal_arn.clone(),
                        crate::types::PullTimeUpdateExclusionData::from(v),
                    )
                })
                .collect(),
            account_settings: view.account_settings,
        }
    }
}

impl From<RepositoryView> for Repository {
    fn from(view: RepositoryView) -> Self {
        let created_at = view
            .created_at
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        Repository {
            repository_name: view.repository_name,
            repository_arn: view.repository_arn,
            repository_uri: view.repository_uri,
            registry_id: view.registry_id,
            created_at,
            images: view.images.into_iter().map(Image::from).collect(),
            tags: view.tags,
            lifecycle_policy: view.lifecycle_policy,
            repository_policy: view.repository_policy,
            image_scanning_configuration: ImageScanningConfiguration::from(
                view.image_scanning_configuration,
            ),
            image_tag_mutability: view.image_tag_mutability,
            encryption_configuration: EncryptionConfiguration {
                encryption_type: view.encryption_configuration.encryption_type,
                kms_key: view.encryption_configuration.kms_key,
            },
        }
    }
}

impl From<ImageView> for Image {
    fn from(view: ImageView) -> Self {
        let pushed_at = view
            .pushed_at
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        Image {
            image_digest: view.image_digest,
            image_tags: view.image_tags,
            image_manifest: view.image_manifest,
            pushed_at,
            image_scan_status: view.image_scan_status.map(ImageScanStatusData::from),
            image_scan_findings: view.image_scan_findings.map(ImageScanFindingsData::from),
        }
    }
}

impl From<ImageScanningConfigView> for ImageScanningConfiguration {
    fn from(view: ImageScanningConfigView) -> Self {
        ImageScanningConfiguration {
            scan_on_push: view.scan_on_push,
        }
    }
}

impl From<ImageScanStatusView> for ImageScanStatusData {
    fn from(view: ImageScanStatusView) -> Self {
        ImageScanStatusData {
            status: view.status,
            description: view.description,
        }
    }
}

impl From<ImageScanFindingsView> for ImageScanFindingsData {
    fn from(view: ImageScanFindingsView) -> Self {
        let image_scan_completed_at = view
            .image_scan_completed_at
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        let vulnerability_source_updated_at = view
            .vulnerability_source_updated_at
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        ImageScanFindingsData {
            image_scan_completed_at,
            vulnerability_source_updated_at,
            finding_severity_counts: view.finding_severity_counts,
        }
    }
}

impl From<RegistryScanningConfigView> for RegistryScanningConfig {
    fn from(view: RegistryScanningConfigView) -> Self {
        RegistryScanningConfig {
            scan_type: view.scan_type,
            rules: view
                .rules
                .into_iter()
                .map(RegistryScanningRuleData::from)
                .collect(),
        }
    }
}

impl From<RegistryScanningRuleView> for RegistryScanningRuleData {
    fn from(view: RegistryScanningRuleView) -> Self {
        RegistryScanningRuleData {
            scan_frequency: view.scan_frequency,
            repository_filters: view
                .repository_filters
                .into_iter()
                .map(ScanningRepositoryFilterData::from)
                .collect(),
        }
    }
}

impl From<ScanningRepositoryFilterView> for ScanningRepositoryFilterData {
    fn from(view: ScanningRepositoryFilterView) -> Self {
        ScanningRepositoryFilterData {
            filter: view.filter,
            filter_type: view.filter_type,
        }
    }
}

impl From<ReplicationConfigView> for ReplicationConfig {
    fn from(view: ReplicationConfigView) -> Self {
        ReplicationConfig {
            rules: view
                .rules
                .into_iter()
                .map(ReplicationRuleData::from)
                .collect(),
        }
    }
}

impl From<ReplicationRuleView> for ReplicationRuleData {
    fn from(view: ReplicationRuleView) -> Self {
        ReplicationRuleData {
            destinations: view
                .destinations
                .into_iter()
                .map(ReplicationDestinationData::from)
                .collect(),
            repository_filters: view
                .repository_filters
                .into_iter()
                .map(RepositoryFilterData::from)
                .collect(),
        }
    }
}

impl From<ReplicationDestinationView> for ReplicationDestinationData {
    fn from(view: ReplicationDestinationView) -> Self {
        ReplicationDestinationData {
            region: view.region,
            registry_id: view.registry_id,
        }
    }
}

impl From<RepositoryFilterView> for RepositoryFilterData {
    fn from(view: RepositoryFilterView) -> Self {
        RepositoryFilterData {
            filter: view.filter,
            filter_type: view.filter_type,
        }
    }
}

// --- From internal types to view types (new state families) ---

impl From<&PullThroughCacheRuleData> for PullThroughCacheRuleView {
    fn from(r: &PullThroughCacheRuleData) -> Self {
        PullThroughCacheRuleView {
            ecr_repository_prefix: r.ecr_repository_prefix.clone(),
            upstream_registry_url: r.upstream_registry_url.clone(),
            registry_id: Some(r.registry_id.clone()),
            created_at: Some(r.created_at.to_rfc3339()),
            upstream_registry: r.upstream_registry.clone(),
            upstream_repository_prefix: r.upstream_repository_prefix.clone(),
            credential_arn: r.credential_arn.clone(),
            custom_role_arn: r.custom_role_arn.clone(),
        }
    }
}

impl From<&RepositoryCreationTemplateData> for RepositoryCreationTemplateView {
    fn from(t: &RepositoryCreationTemplateData) -> Self {
        RepositoryCreationTemplateView {
            prefix: t.prefix.clone(),
            description: t.description.clone(),
            lifecycle_policy: t.lifecycle_policy.clone(),
            repository_policy: t.repository_policy.clone(),
            image_tag_mutability: t.image_tag_mutability.clone(),
            custom_role_arn: t.custom_role_arn.clone(),
            applied_for: t.applied_for.clone(),
            created_at: Some(t.created_at.to_rfc3339()),
            updated_at: Some(t.updated_at.to_rfc3339()),
        }
    }
}

impl From<&SigningConfigData> for SigningConfigView {
    fn from(c: &SigningConfigData) -> Self {
        SigningConfigView {
            rules: c.rules.iter().map(SigningRuleView::from).collect(),
        }
    }
}

impl From<&SigningRuleData> for SigningRuleView {
    fn from(r: &SigningRuleData) -> Self {
        SigningRuleView {
            signing_profile_arn: r.signing_profile_arn.clone(),
            repository_filters: r
                .repository_filters
                .iter()
                .map(SigningRepositoryFilterView::from)
                .collect(),
        }
    }
}

impl From<&SigningRepositoryFilterData> for SigningRepositoryFilterView {
    fn from(f: &SigningRepositoryFilterData) -> Self {
        SigningRepositoryFilterView {
            filter: f.filter.clone(),
            filter_type: f.filter_type.clone(),
        }
    }
}

impl From<&crate::types::PullTimeUpdateExclusionData> for PullTimeUpdateExclusionView {
    fn from(e: &crate::types::PullTimeUpdateExclusionData) -> Self {
        PullTimeUpdateExclusionView {
            principal_arn: e.principal_arn.clone(),
            created_at: Some(e.created_at.to_rfc3339()),
        }
    }
}

// --- From view types to internal types (new state families) ---

impl From<PullThroughCacheRuleView> for PullThroughCacheRuleData {
    fn from(view: PullThroughCacheRuleView) -> Self {
        let created_at = view
            .created_at
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        PullThroughCacheRuleData {
            ecr_repository_prefix: view.ecr_repository_prefix,
            upstream_registry_url: view.upstream_registry_url,
            registry_id: view.registry_id.unwrap_or_default(),
            created_at,
            upstream_registry: view.upstream_registry,
            upstream_repository_prefix: view.upstream_repository_prefix,
            credential_arn: view.credential_arn,
            custom_role_arn: view.custom_role_arn,
        }
    }
}

impl From<RepositoryCreationTemplateView> for RepositoryCreationTemplateData {
    fn from(view: RepositoryCreationTemplateView) -> Self {
        let created_at = view
            .created_at
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        let updated_at = view
            .updated_at
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        RepositoryCreationTemplateData {
            prefix: view.prefix,
            description: view.description,
            lifecycle_policy: view.lifecycle_policy,
            repository_policy: view.repository_policy,
            image_tag_mutability: view.image_tag_mutability,
            custom_role_arn: view.custom_role_arn,
            applied_for: view.applied_for,
            created_at,
            updated_at,
        }
    }
}

impl From<SigningConfigView> for SigningConfigData {
    fn from(view: SigningConfigView) -> Self {
        SigningConfigData {
            rules: view.rules.into_iter().map(SigningRuleData::from).collect(),
        }
    }
}

impl From<SigningRuleView> for SigningRuleData {
    fn from(view: SigningRuleView) -> Self {
        SigningRuleData {
            signing_profile_arn: view.signing_profile_arn,
            repository_filters: view
                .repository_filters
                .into_iter()
                .map(SigningRepositoryFilterData::from)
                .collect(),
        }
    }
}

impl From<SigningRepositoryFilterView> for SigningRepositoryFilterData {
    fn from(view: SigningRepositoryFilterView) -> Self {
        SigningRepositoryFilterData {
            filter: view.filter,
            filter_type: view.filter_type,
        }
    }
}

impl From<PullTimeUpdateExclusionView> for crate::types::PullTimeUpdateExclusionData {
    fn from(view: PullTimeUpdateExclusionView) -> Self {
        let created_at = view
            .created_at
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        crate::types::PullTimeUpdateExclusionData {
            principal_arn: view.principal_arn,
            created_at,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for EcrService {
    type StateView = EcrStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        EcrStateView::from(&*guard)
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
            *guard = EcrState::from(view);
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
            for (name, repo_view) in view.repositories {
                guard.repositories.insert(name, Repository::from(repo_view));
            }
            if let Some(policy) = view.registry_policy {
                guard.registry_policy = Some(policy);
            }
            for (upload_id, upload_view) in view.layer_uploads {
                guard
                    .layer_uploads
                    .insert(upload_id, LayerUpload::from(upload_view));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

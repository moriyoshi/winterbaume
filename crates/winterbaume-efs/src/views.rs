//! Serde-compatible view types for EFS state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::EfsService;
use crate::state::EfsState;
use crate::types::{
    AccessPoint, AccountPreferences, CreationInfo, FileSizeValue, FileSystem, FileSystemProtection,
    LifecyclePolicy, MountTarget, PosixUser, ReplicationConfiguration, ReplicationDestination,
    RootDirectory, Tag,
};

/// Serializable view of the entire EFS state.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EfsStateView {
    /// File systems keyed by file system ID.
    #[serde(default)]
    pub file_systems: HashMap<String, FileSystemView>,
    /// Mount targets keyed by mount target ID.
    #[serde(default)]
    pub mount_targets: HashMap<String, MountTargetView>,
    /// Access points keyed by access point ID.
    #[serde(default)]
    pub access_points: HashMap<String, AccessPointView>,
    /// Maps creation_token -> file_system_id for idempotency.
    #[serde(default)]
    pub creation_tokens: HashMap<String, String>,
    /// Maps client_token -> access_point_id for idempotency.
    #[serde(default)]
    pub access_point_tokens: HashMap<String, String>,
    /// Replication configurations keyed by source file system ID.
    #[serde(default)]
    pub replication_configurations: HashMap<String, ReplicationConfigurationView>,
    /// Account preferences (shared across all accounts in this mock).
    #[serde(default)]
    pub account_preferences: Option<AccountPreferencesView>,
    /// Per-filesystem protection settings keyed by file system ID.
    #[serde(default)]
    pub fs_protection: HashMap<String, FileSystemProtectionView>,
}

/// Serializable view of a replication destination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationDestinationView {
    pub file_system_id: String,
    pub region: String,
    pub status: String,
    pub role_arn: Option<String>,
}

/// Serializable view of a replication configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationConfigurationView {
    pub source_file_system_id: String,
    pub source_file_system_arn: String,
    pub original_source_file_system_arn: String,
    pub source_file_system_region: String,
    pub source_file_system_owner_id: String,
    /// Creation time in RFC 3339 format.
    pub creation_time: Option<String>,
    #[serde(default)]
    pub destinations: Vec<ReplicationDestinationView>,
}

/// Serializable view of account preferences.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountPreferencesView {
    pub resource_id_type: String,
    #[serde(default)]
    pub resources: Vec<String>,
}

/// Serializable view of per-filesystem protection settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSystemProtectionView {
    pub replication_overwrite_protection: Option<String>,
}

/// Serializable view of a single EFS file system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSystemView {
    pub file_system_id: String,
    pub file_system_arn: String,
    /// Creation time in RFC 3339 format.
    pub creation_time: Option<String>,
    pub owner_id: String,
    pub creation_token: String,
    pub performance_mode: String,
    pub throughput_mode: String,
    pub life_cycle_state: String,
    pub number_of_mount_targets: i32,
    pub size_in_bytes: FileSizeValueView,
    pub encrypted: bool,
    #[serde(default)]
    pub tags: Vec<TagView>,
    pub name: Option<String>,
    pub policy: Option<String>,
    #[serde(default)]
    pub lifecycle_policies: Vec<LifecyclePolicyView>,
    pub backup_policy_status: Option<String>,
}

/// Serializable view of a file size value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSizeValueView {
    pub value: i64,
    pub timestamp: Option<String>,
    pub value_in_ia: i64,
    pub value_in_standard: i64,
}

/// Serializable view of a lifecycle policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecyclePolicyView {
    pub transition_to_ia: Option<String>,
    pub transition_to_primary_storage_class: Option<String>,
    pub transition_to_archive: Option<String>,
}

/// Serializable view of an EFS mount target.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountTargetView {
    pub mount_target_id: String,
    pub file_system_id: String,
    pub subnet_id: String,
    pub life_cycle_state: String,
    pub ip_address: String,
    pub network_interface_id: String,
    pub availability_zone_id: String,
    pub availability_zone_name: String,
    pub owner_id: String,
    #[serde(default)]
    pub security_groups: Vec<String>,
}

/// Serializable view of an EFS access point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPointView {
    pub access_point_id: String,
    pub access_point_arn: String,
    pub client_token: String,
    pub file_system_id: String,
    pub life_cycle_state: String,
    pub name: Option<String>,
    pub owner_id: String,
    pub posix_user: Option<PosixUserView>,
    pub root_directory: Option<RootDirectoryView>,
    #[serde(default)]
    pub tags: Vec<TagView>,
}

/// Serializable view of a POSIX user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PosixUserView {
    pub uid: i64,
    pub gid: i64,
    pub secondary_gids: Option<Vec<i64>>,
}

/// Serializable view of a root directory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootDirectoryView {
    pub path: Option<String>,
    pub creation_info: Option<CreationInfoView>,
}

/// Serializable view of creation info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreationInfoView {
    pub owner_uid: i64,
    pub owner_gid: i64,
    pub permissions: String,
}

/// Serializable view of a key-value tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagView {
    pub key: String,
    pub value: String,
}

// --- From internal types to view types ---

impl From<&EfsState> for EfsStateView {
    fn from(state: &EfsState) -> Self {
        EfsStateView {
            file_systems: state
                .file_systems
                .iter()
                .map(|(k, v)| (k.clone(), FileSystemView::from(v)))
                .collect(),
            mount_targets: state
                .mount_targets
                .iter()
                .map(|(k, v)| (k.clone(), MountTargetView::from(v)))
                .collect(),
            access_points: state
                .access_points
                .iter()
                .map(|(k, v)| (k.clone(), AccessPointView::from(v)))
                .collect(),
            creation_tokens: state.creation_tokens.clone(),
            access_point_tokens: state.access_point_tokens.clone(),
            replication_configurations: state
                .replication_configurations
                .iter()
                .map(|(k, v)| (k.clone(), ReplicationConfigurationView::from(v)))
                .collect(),
            account_preferences: state
                .account_preferences
                .as_ref()
                .map(AccountPreferencesView::from),
            fs_protection: state
                .fs_protection
                .iter()
                .map(|(k, v)| (k.clone(), FileSystemProtectionView::from(v)))
                .collect(),
        }
    }
}

impl From<&FileSystem> for FileSystemView {
    fn from(fs: &FileSystem) -> Self {
        FileSystemView {
            file_system_id: fs.file_system_id.clone(),
            file_system_arn: fs.file_system_arn.clone(),
            creation_time: Some(fs.creation_time.to_rfc3339()),
            owner_id: fs.owner_id.clone(),
            creation_token: fs.creation_token.clone(),
            performance_mode: fs.performance_mode.clone(),
            throughput_mode: fs.throughput_mode.clone(),
            life_cycle_state: fs.life_cycle_state.clone(),
            number_of_mount_targets: fs.number_of_mount_targets,
            size_in_bytes: FileSizeValueView::from(&fs.size_in_bytes),
            encrypted: fs.encrypted,
            tags: fs.tags.iter().map(TagView::from).collect(),
            name: fs.name.clone(),
            policy: fs.policy.clone(),
            lifecycle_policies: fs
                .lifecycle_policies
                .iter()
                .map(LifecyclePolicyView::from)
                .collect(),
            backup_policy_status: fs.backup_policy_status.clone(),
        }
    }
}

impl From<&FileSizeValue> for FileSizeValueView {
    fn from(v: &FileSizeValue) -> Self {
        FileSizeValueView {
            value: v.value,
            timestamp: v.timestamp.as_ref().map(|t| t.to_rfc3339()),
            value_in_ia: v.value_in_ia,
            value_in_standard: v.value_in_standard,
        }
    }
}

impl From<&LifecyclePolicy> for LifecyclePolicyView {
    fn from(lp: &LifecyclePolicy) -> Self {
        LifecyclePolicyView {
            transition_to_ia: lp.transition_to_ia.clone(),
            transition_to_primary_storage_class: lp.transition_to_primary_storage_class.clone(),
            transition_to_archive: lp.transition_to_archive.clone(),
        }
    }
}

impl From<&MountTarget> for MountTargetView {
    fn from(mt: &MountTarget) -> Self {
        MountTargetView {
            mount_target_id: mt.mount_target_id.clone(),
            file_system_id: mt.file_system_id.clone(),
            subnet_id: mt.subnet_id.clone(),
            life_cycle_state: mt.life_cycle_state.clone(),
            ip_address: mt.ip_address.clone(),
            network_interface_id: mt.network_interface_id.clone(),
            availability_zone_id: mt.availability_zone_id.clone(),
            availability_zone_name: mt.availability_zone_name.clone(),
            owner_id: mt.owner_id.clone(),
            security_groups: mt.security_groups.clone(),
        }
    }
}

impl From<&AccessPoint> for AccessPointView {
    fn from(ap: &AccessPoint) -> Self {
        AccessPointView {
            access_point_id: ap.access_point_id.clone(),
            access_point_arn: ap.access_point_arn.clone(),
            client_token: ap.client_token.clone(),
            file_system_id: ap.file_system_id.clone(),
            life_cycle_state: ap.life_cycle_state.clone(),
            name: ap.name.clone(),
            owner_id: ap.owner_id.clone(),
            posix_user: ap.posix_user.as_ref().map(PosixUserView::from),
            root_directory: ap.root_directory.as_ref().map(RootDirectoryView::from),
            tags: ap.tags.iter().map(TagView::from).collect(),
        }
    }
}

impl From<&PosixUser> for PosixUserView {
    fn from(p: &PosixUser) -> Self {
        PosixUserView {
            uid: p.uid,
            gid: p.gid,
            secondary_gids: p.secondary_gids.clone(),
        }
    }
}

impl From<&RootDirectory> for RootDirectoryView {
    fn from(rd: &RootDirectory) -> Self {
        RootDirectoryView {
            path: rd.path.clone(),
            creation_info: rd.creation_info.as_ref().map(CreationInfoView::from),
        }
    }
}

impl From<&CreationInfo> for CreationInfoView {
    fn from(ci: &CreationInfo) -> Self {
        CreationInfoView {
            owner_uid: ci.owner_uid,
            owner_gid: ci.owner_gid,
            permissions: ci.permissions.clone(),
        }
    }
}

impl From<&Tag> for TagView {
    fn from(t: &Tag) -> Self {
        TagView {
            key: t.key.clone(),
            value: t.value.clone(),
        }
    }
}

impl From<&ReplicationDestination> for ReplicationDestinationView {
    fn from(rd: &ReplicationDestination) -> Self {
        ReplicationDestinationView {
            file_system_id: rd.file_system_id.clone(),
            region: rd.region.clone(),
            status: rd.status.clone(),
            role_arn: rd.role_arn.clone(),
        }
    }
}

impl From<&ReplicationConfiguration> for ReplicationConfigurationView {
    fn from(rc: &ReplicationConfiguration) -> Self {
        ReplicationConfigurationView {
            source_file_system_id: rc.source_file_system_id.clone(),
            source_file_system_arn: rc.source_file_system_arn.clone(),
            original_source_file_system_arn: rc.original_source_file_system_arn.clone(),
            source_file_system_region: rc.source_file_system_region.clone(),
            source_file_system_owner_id: rc.source_file_system_owner_id.clone(),
            creation_time: Some(rc.creation_time.to_rfc3339()),
            destinations: rc
                .destinations
                .iter()
                .map(ReplicationDestinationView::from)
                .collect(),
        }
    }
}

impl From<&AccountPreferences> for AccountPreferencesView {
    fn from(ap: &AccountPreferences) -> Self {
        AccountPreferencesView {
            resource_id_type: ap.resource_id_type.clone(),
            resources: ap.resources.clone(),
        }
    }
}

impl From<&FileSystemProtection> for FileSystemProtectionView {
    fn from(fsp: &FileSystemProtection) -> Self {
        FileSystemProtectionView {
            replication_overwrite_protection: fsp.replication_overwrite_protection.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<EfsStateView> for EfsState {
    fn from(view: EfsStateView) -> Self {
        EfsState {
            file_systems: view
                .file_systems
                .into_iter()
                .map(|(k, v)| (k, FileSystem::from(v)))
                .collect(),
            mount_targets: view
                .mount_targets
                .into_iter()
                .map(|(k, v)| (k, MountTarget::from(v)))
                .collect(),
            access_points: view
                .access_points
                .into_iter()
                .map(|(k, v)| (k, AccessPoint::from(v)))
                .collect(),
            creation_tokens: view.creation_tokens,
            access_point_tokens: view.access_point_tokens,
            replication_configurations: view
                .replication_configurations
                .into_iter()
                .map(|(k, v)| (k, ReplicationConfiguration::from(v)))
                .collect(),
            account_preferences: view.account_preferences.map(AccountPreferences::from),
            fs_protection: view
                .fs_protection
                .into_iter()
                .map(|(k, v)| (k, FileSystemProtection::from(v)))
                .collect(),
        }
    }
}

impl From<FileSystemView> for FileSystem {
    fn from(view: FileSystemView) -> Self {
        let creation_time = view
            .creation_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        FileSystem {
            file_system_id: view.file_system_id,
            file_system_arn: view.file_system_arn,
            creation_time,
            owner_id: view.owner_id,
            creation_token: view.creation_token,
            performance_mode: view.performance_mode,
            throughput_mode: view.throughput_mode,
            life_cycle_state: view.life_cycle_state,
            number_of_mount_targets: view.number_of_mount_targets,
            size_in_bytes: FileSizeValue::from(view.size_in_bytes),
            encrypted: view.encrypted,
            tags: view.tags.into_iter().map(Tag::from).collect(),
            name: view.name,
            policy: view.policy,
            lifecycle_policies: view
                .lifecycle_policies
                .into_iter()
                .map(LifecyclePolicy::from)
                .collect(),
            backup_policy_status: view.backup_policy_status,
        }
    }
}

impl From<FileSizeValueView> for FileSizeValue {
    fn from(view: FileSizeValueView) -> Self {
        let timestamp = view
            .timestamp
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
        FileSizeValue {
            value: view.value,
            timestamp,
            value_in_ia: view.value_in_ia,
            value_in_standard: view.value_in_standard,
        }
    }
}

impl From<LifecyclePolicyView> for LifecyclePolicy {
    fn from(view: LifecyclePolicyView) -> Self {
        LifecyclePolicy {
            transition_to_ia: view.transition_to_ia,
            transition_to_primary_storage_class: view.transition_to_primary_storage_class,
            transition_to_archive: view.transition_to_archive,
        }
    }
}

impl From<MountTargetView> for MountTarget {
    fn from(view: MountTargetView) -> Self {
        MountTarget {
            mount_target_id: view.mount_target_id,
            file_system_id: view.file_system_id,
            subnet_id: view.subnet_id,
            life_cycle_state: view.life_cycle_state,
            ip_address: view.ip_address,
            network_interface_id: view.network_interface_id,
            availability_zone_id: view.availability_zone_id,
            availability_zone_name: view.availability_zone_name,
            owner_id: view.owner_id,
            security_groups: view.security_groups,
        }
    }
}

impl From<AccessPointView> for AccessPoint {
    fn from(view: AccessPointView) -> Self {
        AccessPoint {
            access_point_id: view.access_point_id,
            access_point_arn: view.access_point_arn,
            client_token: view.client_token,
            file_system_id: view.file_system_id,
            life_cycle_state: view.life_cycle_state,
            name: view.name,
            owner_id: view.owner_id,
            posix_user: view.posix_user.map(PosixUser::from),
            root_directory: view.root_directory.map(RootDirectory::from),
            tags: view.tags.into_iter().map(Tag::from).collect(),
        }
    }
}

impl From<PosixUserView> for PosixUser {
    fn from(view: PosixUserView) -> Self {
        PosixUser {
            uid: view.uid,
            gid: view.gid,
            secondary_gids: view.secondary_gids,
        }
    }
}

impl From<RootDirectoryView> for RootDirectory {
    fn from(view: RootDirectoryView) -> Self {
        RootDirectory {
            path: view.path,
            creation_info: view.creation_info.map(CreationInfo::from),
        }
    }
}

impl From<CreationInfoView> for CreationInfo {
    fn from(view: CreationInfoView) -> Self {
        CreationInfo {
            owner_uid: view.owner_uid,
            owner_gid: view.owner_gid,
            permissions: view.permissions,
        }
    }
}

impl From<TagView> for Tag {
    fn from(view: TagView) -> Self {
        Tag {
            key: view.key,
            value: view.value,
        }
    }
}

impl From<ReplicationDestinationView> for ReplicationDestination {
    fn from(view: ReplicationDestinationView) -> Self {
        ReplicationDestination {
            file_system_id: view.file_system_id,
            region: view.region,
            status: view.status,
            role_arn: view.role_arn,
        }
    }
}

impl From<ReplicationConfigurationView> for ReplicationConfiguration {
    fn from(view: ReplicationConfigurationView) -> Self {
        use chrono::DateTime;
        let creation_time = view
            .creation_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        ReplicationConfiguration {
            source_file_system_id: view.source_file_system_id,
            source_file_system_arn: view.source_file_system_arn,
            original_source_file_system_arn: view.original_source_file_system_arn,
            source_file_system_region: view.source_file_system_region,
            source_file_system_owner_id: view.source_file_system_owner_id,
            creation_time,
            destinations: view
                .destinations
                .into_iter()
                .map(ReplicationDestination::from)
                .collect(),
        }
    }
}

impl From<AccountPreferencesView> for AccountPreferences {
    fn from(view: AccountPreferencesView) -> Self {
        AccountPreferences {
            resource_id_type: view.resource_id_type,
            resources: view.resources,
        }
    }
}

impl From<FileSystemProtectionView> for FileSystemProtection {
    fn from(view: FileSystemProtectionView) -> Self {
        FileSystemProtection {
            replication_overwrite_protection: view.replication_overwrite_protection,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for EfsService {
    type StateView = EfsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        EfsStateView::from(&*guard)
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
            *guard = EfsState::from(view);
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
            for (id, fs_view) in view.file_systems {
                guard.file_systems.insert(id, FileSystem::from(fs_view));
            }
            for (id, mt_view) in view.mount_targets {
                guard.mount_targets.insert(id, MountTarget::from(mt_view));
            }
            for (id, ap_view) in view.access_points {
                guard.access_points.insert(id, AccessPoint::from(ap_view));
            }
            for (token, fs_id) in view.creation_tokens {
                guard.creation_tokens.insert(token, fs_id);
            }
            for (token, ap_id) in view.access_point_tokens {
                guard.access_point_tokens.insert(token, ap_id);
            }
            for (id, rc_view) in view.replication_configurations {
                guard
                    .replication_configurations
                    .insert(id, ReplicationConfiguration::from(rc_view));
            }
            if let Some(prefs_view) = view.account_preferences {
                guard.account_preferences = Some(AccountPreferences::from(prefs_view));
            }
            for (id, prot_view) in view.fs_protection {
                guard
                    .fs_protection
                    .insert(id, FileSystemProtection::from(prot_view));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

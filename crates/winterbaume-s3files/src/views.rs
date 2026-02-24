//! Serde-compatible view types for S3 Files state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::S3FilesService;
use crate::state::S3FilesState;
use crate::types::{
    AccessPoint, CreationPermissions, ExpirationDataRule, FileSystem, ImportDataRule, MountTarget,
    PosixUser, RootDirectory, SynchronizationConfiguration,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct S3FilesStateView {
    #[serde(default)]
    pub file_systems: HashMap<String, FileSystemView>,
    #[serde(default)]
    pub mount_targets: HashMap<String, MountTargetView>,
    #[serde(default)]
    pub access_points: HashMap<String, AccessPointView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSystemView {
    pub file_system_id: String,
    pub file_system_arn: String,
    pub bucket: String,
    pub prefix: Option<String>,
    pub kms_key_id: Option<String>,
    pub role_arn: String,
    pub client_token: Option<String>,
    pub name: Option<String>,
    pub status: String,
    pub status_message: Option<String>,
    pub creation_time: DateTime<Utc>,
    pub owner_id: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub policy: Option<String>,
    #[serde(default)]
    pub synchronization_configuration: SynchronizationConfigurationView,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SynchronizationConfigurationView {
    #[serde(default)]
    pub latest_version_number: i32,
    #[serde(default)]
    pub import_data_rules: Vec<ImportDataRuleView>,
    #[serde(default)]
    pub expiration_data_rules: Vec<ExpirationDataRuleView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportDataRuleView {
    pub prefix: String,
    pub size_less_than: i64,
    pub trigger: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpirationDataRuleView {
    pub days_after_last_access: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountTargetView {
    pub mount_target_id: String,
    pub file_system_id: String,
    pub subnet_id: String,
    pub vpc_id: String,
    pub availability_zone_id: String,
    pub ipv4_address: Option<String>,
    pub ipv6_address: Option<String>,
    pub ip_address_type: String,
    pub network_interface_id: String,
    #[serde(default)]
    pub security_groups: Vec<String>,
    pub status: String,
    pub status_message: Option<String>,
    pub owner_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPointView {
    pub access_point_id: String,
    pub access_point_arn: String,
    pub file_system_id: String,
    pub name: Option<String>,
    pub posix_user: Option<PosixUserView>,
    pub root_directory: Option<RootDirectoryView>,
    pub status: String,
    pub owner_id: String,
    pub client_token: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PosixUserView {
    pub uid: i64,
    pub gid: i64,
    #[serde(default)]
    pub secondary_gids: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootDirectoryView {
    pub path: Option<String>,
    pub creation_permissions: Option<CreationPermissionsView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreationPermissionsView {
    pub owner_uid: i64,
    pub owner_gid: i64,
    pub permissions: String,
}

// ── conversions: domain -> view ──────────────────────────────────────────

impl From<&FileSystem> for FileSystemView {
    fn from(fs: &FileSystem) -> Self {
        Self {
            file_system_id: fs.file_system_id.clone(),
            file_system_arn: fs.file_system_arn.clone(),
            bucket: fs.bucket.clone(),
            prefix: fs.prefix.clone(),
            kms_key_id: fs.kms_key_id.clone(),
            role_arn: fs.role_arn.clone(),
            client_token: fs.client_token.clone(),
            name: fs.name.clone(),
            status: fs.status.clone(),
            status_message: fs.status_message.clone(),
            creation_time: fs.creation_time,
            owner_id: fs.owner_id.clone(),
            tags: fs.tags.clone(),
            policy: fs.policy.clone(),
            synchronization_configuration: SynchronizationConfigurationView::from(
                &fs.synchronization_configuration,
            ),
        }
    }
}

impl From<&SynchronizationConfiguration> for SynchronizationConfigurationView {
    fn from(s: &SynchronizationConfiguration) -> Self {
        Self {
            latest_version_number: s.latest_version_number,
            import_data_rules: s.import_data_rules.iter().map(Into::into).collect(),
            expiration_data_rules: s.expiration_data_rules.iter().map(Into::into).collect(),
        }
    }
}

impl From<&ImportDataRule> for ImportDataRuleView {
    fn from(r: &ImportDataRule) -> Self {
        Self {
            prefix: r.prefix.clone(),
            size_less_than: r.size_less_than,
            trigger: r.trigger.clone(),
        }
    }
}

impl From<&ExpirationDataRule> for ExpirationDataRuleView {
    fn from(r: &ExpirationDataRule) -> Self {
        Self {
            days_after_last_access: r.days_after_last_access,
        }
    }
}

impl From<&MountTarget> for MountTargetView {
    fn from(mt: &MountTarget) -> Self {
        Self {
            mount_target_id: mt.mount_target_id.clone(),
            file_system_id: mt.file_system_id.clone(),
            subnet_id: mt.subnet_id.clone(),
            vpc_id: mt.vpc_id.clone(),
            availability_zone_id: mt.availability_zone_id.clone(),
            ipv4_address: mt.ipv4_address.clone(),
            ipv6_address: mt.ipv6_address.clone(),
            ip_address_type: mt.ip_address_type.clone(),
            network_interface_id: mt.network_interface_id.clone(),
            security_groups: mt.security_groups.clone(),
            status: mt.status.clone(),
            status_message: mt.status_message.clone(),
            owner_id: mt.owner_id.clone(),
        }
    }
}

impl From<&AccessPoint> for AccessPointView {
    fn from(ap: &AccessPoint) -> Self {
        Self {
            access_point_id: ap.access_point_id.clone(),
            access_point_arn: ap.access_point_arn.clone(),
            file_system_id: ap.file_system_id.clone(),
            name: ap.name.clone(),
            posix_user: ap.posix_user.as_ref().map(Into::into),
            root_directory: ap.root_directory.as_ref().map(Into::into),
            status: ap.status.clone(),
            owner_id: ap.owner_id.clone(),
            client_token: ap.client_token.clone(),
            tags: ap.tags.clone(),
        }
    }
}

impl From<&PosixUser> for PosixUserView {
    fn from(p: &PosixUser) -> Self {
        Self {
            uid: p.uid,
            gid: p.gid,
            secondary_gids: p.secondary_gids.clone(),
        }
    }
}

impl From<&RootDirectory> for RootDirectoryView {
    fn from(r: &RootDirectory) -> Self {
        Self {
            path: r.path.clone(),
            creation_permissions: r.creation_permissions.as_ref().map(Into::into),
        }
    }
}

impl From<&CreationPermissions> for CreationPermissionsView {
    fn from(c: &CreationPermissions) -> Self {
        Self {
            owner_uid: c.owner_uid,
            owner_gid: c.owner_gid,
            permissions: c.permissions.clone(),
        }
    }
}

impl From<&S3FilesState> for S3FilesStateView {
    fn from(state: &S3FilesState) -> Self {
        Self {
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
        }
    }
}

// ── conversions: view -> domain ──────────────────────────────────────────

impl From<FileSystemView> for FileSystem {
    fn from(view: FileSystemView) -> Self {
        Self {
            file_system_id: view.file_system_id,
            file_system_arn: view.file_system_arn,
            bucket: view.bucket,
            prefix: view.prefix,
            kms_key_id: view.kms_key_id,
            role_arn: view.role_arn,
            client_token: view.client_token,
            name: view.name,
            status: view.status,
            status_message: view.status_message,
            creation_time: view.creation_time,
            owner_id: view.owner_id,
            tags: view.tags,
            policy: view.policy,
            synchronization_configuration: view.synchronization_configuration.into(),
        }
    }
}

impl From<SynchronizationConfigurationView> for SynchronizationConfiguration {
    fn from(v: SynchronizationConfigurationView) -> Self {
        Self {
            latest_version_number: v.latest_version_number,
            import_data_rules: v.import_data_rules.into_iter().map(Into::into).collect(),
            expiration_data_rules: v
                .expiration_data_rules
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl From<ImportDataRuleView> for ImportDataRule {
    fn from(v: ImportDataRuleView) -> Self {
        Self {
            prefix: v.prefix,
            size_less_than: v.size_less_than,
            trigger: v.trigger,
        }
    }
}

impl From<ExpirationDataRuleView> for ExpirationDataRule {
    fn from(v: ExpirationDataRuleView) -> Self {
        Self {
            days_after_last_access: v.days_after_last_access,
        }
    }
}

impl From<MountTargetView> for MountTarget {
    fn from(v: MountTargetView) -> Self {
        Self {
            mount_target_id: v.mount_target_id,
            file_system_id: v.file_system_id,
            subnet_id: v.subnet_id,
            vpc_id: v.vpc_id,
            availability_zone_id: v.availability_zone_id,
            ipv4_address: v.ipv4_address,
            ipv6_address: v.ipv6_address,
            ip_address_type: v.ip_address_type,
            network_interface_id: v.network_interface_id,
            security_groups: v.security_groups,
            status: v.status,
            status_message: v.status_message,
            owner_id: v.owner_id,
        }
    }
}

impl From<AccessPointView> for AccessPoint {
    fn from(v: AccessPointView) -> Self {
        Self {
            access_point_id: v.access_point_id,
            access_point_arn: v.access_point_arn,
            file_system_id: v.file_system_id,
            name: v.name,
            posix_user: v.posix_user.map(Into::into),
            root_directory: v.root_directory.map(Into::into),
            status: v.status,
            owner_id: v.owner_id,
            client_token: v.client_token,
            tags: v.tags,
        }
    }
}

impl From<PosixUserView> for PosixUser {
    fn from(v: PosixUserView) -> Self {
        Self {
            uid: v.uid,
            gid: v.gid,
            secondary_gids: v.secondary_gids,
        }
    }
}

impl From<RootDirectoryView> for RootDirectory {
    fn from(v: RootDirectoryView) -> Self {
        Self {
            path: v.path,
            creation_permissions: v.creation_permissions.map(Into::into),
        }
    }
}

impl From<CreationPermissionsView> for CreationPermissions {
    fn from(v: CreationPermissionsView) -> Self {
        Self {
            owner_uid: v.owner_uid,
            owner_gid: v.owner_gid,
            permissions: v.permissions,
        }
    }
}

impl From<S3FilesStateView> for S3FilesState {
    fn from(view: S3FilesStateView) -> Self {
        Self {
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
        }
    }
}

impl StatefulService for S3FilesService {
    type StateView = S3FilesStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        S3FilesStateView::from(&*guard)
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
            *guard = S3FilesState::from(view);
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
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

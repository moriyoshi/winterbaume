//! Serde-compatible view types for FSx state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::FsxService;
use crate::state::FsxState;
use crate::types::{Backup, FileSystem, Tag};

/// Serializable view of the entire FSx state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FsxStateView {
    #[serde(default)]
    pub file_systems: HashMap<String, FileSystemView>,
    #[serde(default)]
    pub backups: HashMap<String, BackupView>,
    /// Tags stored per resource ARN
    #[serde(default)]
    pub tags: HashMap<String, Vec<TagView>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSystemView {
    pub file_system_id: String,
    pub file_system_type: String,
    pub storage_capacity: i64,
    pub storage_type: String,
    #[serde(default)]
    pub subnet_ids: Vec<String>,
    #[serde(default)]
    pub security_group_ids: Vec<String>,
    pub dns_name: String,
    pub kms_key_id: Option<String>,
    pub resource_arn: String,
    #[serde(default)]
    pub tags: Vec<TagView>,
    pub windows_configuration: Option<Value>,
    pub lustre_configuration: Option<Value>,
    pub ontap_configuration: Option<Value>,
    pub open_zfs_configuration: Option<Value>,
    pub creation_time: Option<String>,
    #[serde(default)]
    pub lifecycle: String,
    pub owner_id: Option<String>,
    pub vpc_id: Option<String>,
    pub deployment_type: Option<String>,
    #[serde(default)]
    pub copy_tags_to_backups: bool,
    #[serde(default)]
    pub automatic_backup_retention_days: i32,
    pub daily_automatic_backup_start_time: Option<String>,
    pub weekly_maintenance_start_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupView {
    pub backup_id: String,
    pub file_system_id: String,
    pub lifecycle: String,
    pub creation_time: String,
    pub resource_arn: String,
    #[serde(default)]
    pub tags: Vec<TagView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagView {
    pub key: String,
    pub value: String,
}

// ---------------------------------------------------------------------------
// From conversions
// ---------------------------------------------------------------------------

impl From<&Tag> for TagView {
    fn from(t: &Tag) -> Self {
        TagView {
            key: t.key.clone(),
            value: t.value.clone(),
        }
    }
}

impl From<TagView> for Tag {
    fn from(tv: TagView) -> Self {
        Tag {
            key: tv.key,
            value: tv.value,
        }
    }
}

impl From<&FileSystem> for FileSystemView {
    fn from(fs: &FileSystem) -> Self {
        FileSystemView {
            file_system_id: fs.file_system_id.clone(),
            file_system_type: fs.file_system_type.clone(),
            storage_capacity: fs.storage_capacity,
            storage_type: fs.storage_type.clone(),
            subnet_ids: fs.subnet_ids.clone(),
            security_group_ids: fs.security_group_ids.clone(),
            dns_name: fs.dns_name.clone(),
            kms_key_id: fs.kms_key_id.clone(),
            resource_arn: fs.resource_arn.clone(),
            tags: fs.tags.iter().map(TagView::from).collect(),
            windows_configuration: fs.windows_configuration.clone(),
            lustre_configuration: fs.lustre_configuration.clone(),
            ontap_configuration: fs.ontap_configuration.clone(),
            open_zfs_configuration: fs.open_zfs_configuration.clone(),
            creation_time: fs.creation_time.clone(),
            lifecycle: fs.lifecycle.clone(),
            owner_id: fs.owner_id.clone(),
            vpc_id: fs.vpc_id.clone(),
            deployment_type: fs.deployment_type.clone(),
            copy_tags_to_backups: fs.copy_tags_to_backups,
            automatic_backup_retention_days: fs.automatic_backup_retention_days,
            daily_automatic_backup_start_time: fs.daily_automatic_backup_start_time.clone(),
            weekly_maintenance_start_time: fs.weekly_maintenance_start_time.clone(),
        }
    }
}

impl From<&Backup> for BackupView {
    fn from(b: &Backup) -> Self {
        BackupView {
            backup_id: b.backup_id.clone(),
            file_system_id: b.file_system_id.clone(),
            lifecycle: b.lifecycle.clone(),
            creation_time: b.creation_time.clone(),
            resource_arn: b.resource_arn.clone(),
            tags: b.tags.iter().map(TagView::from).collect(),
        }
    }
}

// ---------------------------------------------------------------------------
// StatefulService implementation
// ---------------------------------------------------------------------------

impl StatefulService for FsxService {
    type StateView = FsxStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;

        let file_systems = guard
            .file_systems
            .values()
            .map(|fs| (fs.file_system_id.clone(), FileSystemView::from(fs)))
            .collect();

        let backups = guard
            .backups
            .values()
            .map(|b| (b.backup_id.clone(), BackupView::from(b)))
            .collect();

        let tags = guard
            .tags
            .iter()
            .map(|(arn, tags)| (arn.clone(), tags.iter().map(TagView::from).collect()))
            .collect();

        FsxStateView {
            file_systems,
            backups,
            tags,
        }
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let mut new_state = FsxState::default();

        for (id, fsv) in view.file_systems {
            new_state.file_systems.insert(
                id,
                FileSystem {
                    file_system_id: fsv.file_system_id,
                    file_system_type: fsv.file_system_type,
                    storage_capacity: fsv.storage_capacity,
                    storage_type: fsv.storage_type,
                    subnet_ids: fsv.subnet_ids,
                    security_group_ids: fsv.security_group_ids,
                    dns_name: fsv.dns_name,
                    kms_key_id: fsv.kms_key_id,
                    resource_arn: fsv.resource_arn,
                    tags: fsv.tags.into_iter().map(Tag::from).collect(),
                    windows_configuration: fsv.windows_configuration,
                    lustre_configuration: fsv.lustre_configuration,
                    ontap_configuration: fsv.ontap_configuration,
                    open_zfs_configuration: fsv.open_zfs_configuration,
                    creation_time: fsv.creation_time,
                    lifecycle: fsv.lifecycle,
                    owner_id: fsv.owner_id,
                    vpc_id: fsv.vpc_id,
                    deployment_type: fsv.deployment_type,
                    copy_tags_to_backups: fsv.copy_tags_to_backups,
                    automatic_backup_retention_days: fsv.automatic_backup_retention_days,
                    daily_automatic_backup_start_time: fsv.daily_automatic_backup_start_time,
                    weekly_maintenance_start_time: fsv.weekly_maintenance_start_time,
                },
            );
        }

        for (id, bv) in view.backups {
            new_state.backups.insert(
                id,
                Backup {
                    backup_id: bv.backup_id,
                    file_system_id: bv.file_system_id,
                    lifecycle: bv.lifecycle,
                    creation_time: bv.creation_time,
                    resource_arn: bv.resource_arn,
                    tags: bv.tags.into_iter().map(Tag::from).collect(),
                },
            );
        }

        for (arn, tags) in view.tags {
            new_state
                .tags
                .insert(arn, tags.into_iter().map(Tag::from).collect());
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
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;

            for (id, fsv) in view.file_systems {
                guard.file_systems.insert(
                    id,
                    FileSystem {
                        file_system_id: fsv.file_system_id,
                        file_system_type: fsv.file_system_type,
                        storage_capacity: fsv.storage_capacity,
                        storage_type: fsv.storage_type,
                        subnet_ids: fsv.subnet_ids,
                        security_group_ids: fsv.security_group_ids,
                        dns_name: fsv.dns_name,
                        kms_key_id: fsv.kms_key_id,
                        resource_arn: fsv.resource_arn,
                        tags: fsv.tags.into_iter().map(Tag::from).collect(),
                        windows_configuration: fsv.windows_configuration,
                        lustre_configuration: fsv.lustre_configuration,
                        ontap_configuration: fsv.ontap_configuration,
                        open_zfs_configuration: fsv.open_zfs_configuration,
                        creation_time: fsv.creation_time,
                        lifecycle: fsv.lifecycle,
                        owner_id: fsv.owner_id,
                        vpc_id: fsv.vpc_id,
                        deployment_type: fsv.deployment_type,
                        copy_tags_to_backups: fsv.copy_tags_to_backups,
                        automatic_backup_retention_days: fsv.automatic_backup_retention_days,
                        daily_automatic_backup_start_time: fsv.daily_automatic_backup_start_time,
                        weekly_maintenance_start_time: fsv.weekly_maintenance_start_time,
                    },
                );
            }

            for (id, bv) in view.backups {
                guard.backups.insert(
                    id,
                    Backup {
                        backup_id: bv.backup_id,
                        file_system_id: bv.file_system_id,
                        lifecycle: bv.lifecycle,
                        creation_time: bv.creation_time,
                        resource_arn: bv.resource_arn,
                        tags: bv.tags.into_iter().map(Tag::from).collect(),
                    },
                );
            }

            for (arn, tags) in view.tags {
                guard
                    .tags
                    .insert(arn, tags.into_iter().map(Tag::from).collect());
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

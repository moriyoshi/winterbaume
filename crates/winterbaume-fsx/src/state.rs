use std::collections::HashMap;

use serde_json::Value;
use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct FsxState {
    pub file_systems: HashMap<String, FileSystem>,
    pub backups: HashMap<String, Backup>,
    /// Tags stored per resource ARN
    pub tags: HashMap<String, Vec<Tag>>,
}

#[derive(Debug, Error)]
pub enum FsxError {
    #[error("File system {file_system_id} not found")]
    FileSystemNotFound { file_system_id: String },

    #[error("FSx resource, {resource_id} does not exist")]
    ResourceNotFound { resource_id: String },
}

impl FsxState {
    pub fn create_file_system(
        &mut self,
        file_system_type: &str,
        storage_capacity: i64,
        storage_type: &str,
        subnet_ids: Vec<String>,
        security_group_ids: Vec<String>,
        tags: Option<Vec<Tag>>,
        kms_key_id: Option<String>,
        windows_configuration: Option<Value>,
        lustre_configuration: Option<Value>,
        ontap_configuration: Option<Value>,
        open_zfs_configuration: Option<Value>,
        account_id: &str,
        region: &str,
    ) -> &FileSystem {
        let file_system_id = format!("fs-{}", &uuid::Uuid::new_v4().simple().to_string()[..8]);
        let dns_name = format!("{file_system_id}.fsx.{region}.amazonaws.com");
        let resource_arn =
            format!("arn:aws:fsx:{region}:{account_id}:file-system/{file_system_id}");

        let file_tags = tags.clone().unwrap_or_default();

        let fs = FileSystem {
            file_system_id: file_system_id.clone(),
            file_system_type: file_system_type.to_string(),
            storage_capacity,
            storage_type: storage_type.to_string(),
            subnet_ids,
            security_group_ids,
            dns_name,
            kms_key_id,
            resource_arn: resource_arn.clone(),
            tags: file_tags.clone(),
            windows_configuration,
            lustre_configuration,
            ontap_configuration,
            open_zfs_configuration,
            creation_time: Some(chrono::Utc::now().to_rfc3339()),
            lifecycle: "AVAILABLE".to_string(),
            owner_id: Some(account_id.to_string()),
            vpc_id: None,
            deployment_type: None,
            copy_tags_to_backups: false,
            automatic_backup_retention_days: 0,
            daily_automatic_backup_start_time: None,
            weekly_maintenance_start_time: None,
        };

        if !file_tags.is_empty() {
            self.tags.insert(resource_arn.clone(), file_tags);
        }

        self.file_systems.insert(file_system_id.clone(), fs);
        self.file_systems.get(&file_system_id).unwrap()
    }

    pub fn describe_file_systems(&self, file_system_ids: Option<&[String]>) -> Vec<&FileSystem> {
        match file_system_ids {
            Some(ids) if !ids.is_empty() => ids
                .iter()
                .filter_map(|id| self.file_systems.get(id))
                .collect(),
            _ => self.file_systems.values().collect(),
        }
    }

    pub fn delete_file_system(
        &mut self,
        file_system_id: &str,
    ) -> Result<(String, String, Option<Value>, Option<Value>, Option<Value>), FsxError> {
        let fs = self.file_systems.remove(file_system_id).ok_or_else(|| {
            FsxError::FileSystemNotFound {
                file_system_id: file_system_id.to_string(),
            }
        })?;

        let response_template = serde_json::json!({
            "FinalBackUpId": "",
            "FinalBackUpTags": [],
        });

        let windows_response = if fs.file_system_type == "WINDOWS" {
            Some(response_template.clone())
        } else {
            None
        };
        let lustre_response = if fs.file_system_type == "LUSTRE" {
            Some(response_template.clone())
        } else {
            None
        };
        let open_zfs_response = if fs.file_system_type == "OPEN_ZFS" {
            Some(response_template)
        } else {
            None
        };

        Ok((
            file_system_id.to_string(),
            "DELETING".to_string(),
            windows_response,
            lustre_response,
            open_zfs_response,
        ))
    }

    pub fn create_backup(
        &mut self,
        file_system_id: &str,
        tags: Option<Vec<Tag>>,
        account_id: &str,
        region: &str,
    ) -> Result<&Backup, FsxError> {
        if !self.file_systems.contains_key(file_system_id) {
            return Err(FsxError::ResourceNotFound {
                resource_id: file_system_id.to_string(),
            });
        }

        let backup_id = format!("backup-{}", &uuid::Uuid::new_v4().simple().to_string()[..8]);
        let resource_arn = format!("arn:aws:fsx:{region}:{account_id}:backup/{backup_id}");

        let backup_tags = tags.clone().unwrap_or_default();

        let backup = Backup {
            backup_id: backup_id.clone(),
            file_system_id: file_system_id.to_string(),
            lifecycle: "CREATING".to_string(),
            creation_time: chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            resource_arn: resource_arn.clone(),
            tags: backup_tags.clone(),
        };

        if !backup_tags.is_empty() {
            self.tags.insert(resource_arn, backup_tags);
        }

        self.backups.insert(backup_id.clone(), backup);
        Ok(self.backups.get(&backup_id).unwrap())
    }

    pub fn delete_backup(&mut self, backup_id: &str) -> Result<(String, String), FsxError> {
        if self.backups.remove(backup_id).is_none() {
            return Err(FsxError::ResourceNotFound {
                resource_id: backup_id.to_string(),
            });
        }
        Ok((backup_id.to_string(), "DELETED".to_string()))
    }

    pub fn describe_backups(
        &self,
        backup_ids: Option<&[String]>,
        filters: Option<&[Filter]>,
    ) -> Vec<&Backup> {
        let mut backups: Vec<&Backup> = match backup_ids {
            Some(ids) if !ids.is_empty() => {
                ids.iter().filter_map(|id| self.backups.get(id)).collect()
            }
            _ => self.backups.values().collect(),
        };

        if let Some(filters) = filters {
            for filter in filters {
                if filter.name == "file-system-id" {
                    backups.retain(|b| filter.values.contains(&b.file_system_id));
                }
            }
        }

        backups
    }

    pub fn tag_resource(&mut self, resource_arn: &str, new_tags: Vec<Tag>) {
        let tags = self.tags.entry(resource_arn.to_string()).or_default();
        for new_tag in new_tags {
            if let Some(existing) = tags.iter_mut().find(|t| t.key == new_tag.key) {
                existing.value = new_tag.value;
            } else {
                tags.push(new_tag);
            }
        }
        // Update file system tags if applicable
        for fs in self.file_systems.values_mut() {
            if fs.resource_arn == resource_arn {
                fs.tags = tags.clone();
            }
        }
    }

    pub fn untag_resource(&mut self, resource_arn: &str, tag_keys: &[String]) {
        if let Some(tags) = self.tags.get_mut(resource_arn) {
            tags.retain(|t| !tag_keys.contains(&t.key));
            let updated_tags = tags.clone();
            // Update file system tags if applicable
            for fs in self.file_systems.values_mut() {
                if fs.resource_arn == resource_arn {
                    fs.tags = updated_tags.clone();
                }
            }
        }
    }

    pub fn list_tags_for_resource(&self, resource_arn: &str) -> Vec<&Tag> {
        self.tags
            .get(resource_arn)
            .map(|tags| tags.iter().collect())
            .unwrap_or_default()
    }
}

pub struct Filter {
    pub name: String,
    pub values: Vec<String>,
}

use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct EfsState {
    pub file_systems: HashMap<String, FileSystem>,
    pub mount_targets: HashMap<String, MountTarget>,
    pub access_points: HashMap<String, AccessPoint>,
    /// Maps creation_token -> file_system_id for idempotency.
    pub creation_tokens: HashMap<String, String>,
    /// Maps client_token -> access_point_id for idempotency.
    pub access_point_tokens: HashMap<String, String>,
    /// Maps source_file_system_id -> ReplicationConfiguration.
    pub replication_configurations: HashMap<String, ReplicationConfiguration>,
    /// Account preferences (shared across all accounts in this mock).
    pub account_preferences: Option<AccountPreferences>,
    /// Per-filesystem protection settings.
    pub fs_protection: HashMap<String, FileSystemProtection>,
}

#[derive(Debug, thiserror::Error)]
pub enum EfsError {
    #[error("File system with creation token '{0}' already exists.")]
    FileSystemAlreadyExists(String),
    #[error("File system '{0}' does not exist.")]
    FileSystemNotFound(String),
    #[error("File system '{0}' has mount targets and cannot be deleted.")]
    FileSystemInUse(String),
    #[error("Mount target '{0}' does not exist.")]
    MountTargetNotFound(String),
    #[error("Must specify FileSystemId or MountTargetId")]
    MissingMountTargetFilter,
    #[error("Access point '{0}' does not exist.")]
    AccessPointNotFound(String),
    #[error("No policy found for file system '{0}'.")]
    PolicyNotFound(String),
    #[error("Replication configuration already exists for file system '{0}'.")]
    ReplicationAlreadyExists(String),
    #[error("Replication configuration for '{0}' does not exist.")]
    ReplicationNotFound(String),
    #[error("Resource '{0}' does not exist.")]
    ResourceNotFound(String),
}

impl EfsState {
    pub fn create_file_system(
        &mut self,
        creation_token: &str,
        owner_id: &str,
        region: &str,
        performance_mode: Option<&str>,
        throughput_mode: Option<&str>,
        encrypted: Option<bool>,
        tags: Vec<Tag>,
    ) -> Result<FileSystem, EfsError> {
        // If the same creation token was used, return FileSystemAlreadyExists error
        if let Some(fs_id) = self.creation_tokens.get(creation_token) {
            if self.file_systems.contains_key(fs_id) {
                return Err(EfsError::FileSystemAlreadyExists(
                    creation_token.to_string(),
                ));
            }
        }

        let fs_id = format!(
            "fs-{}",
            &uuid::Uuid::new_v4().to_string().replace('-', "")[..17]
        );
        let fs_arn = format!("arn:aws:elasticfilesystem:{region}:{owner_id}:file-system/{fs_id}");

        let name = tags
            .iter()
            .find(|t| t.key == "Name")
            .map(|t| t.value.clone())
            .unwrap_or_default();

        let performance_mode = performance_mode.unwrap_or("generalPurpose").to_string();
        let throughput_mode = throughput_mode.unwrap_or("bursting").to_string();
        let encrypted = encrypted.unwrap_or(false);

        let fs = FileSystem {
            file_system_id: fs_id.clone(),
            file_system_arn: fs_arn,
            creation_time: Utc::now(),
            owner_id: owner_id.to_string(),
            creation_token: creation_token.to_string(),
            performance_mode,
            throughput_mode,
            life_cycle_state: "available".to_string(),
            number_of_mount_targets: 0,
            size_in_bytes: FileSizeValue {
                value: 0,
                timestamp: Some(Utc::now()),
                value_in_ia: 0,
                value_in_standard: 0,
            },
            encrypted,
            tags,
            name: Some(name),
            policy: None,
            lifecycle_policies: Vec::new(),
            backup_policy_status: None,
        };

        self.creation_tokens
            .insert(creation_token.to_string(), fs_id.clone());
        self.file_systems.insert(fs_id.clone(), fs.clone());
        Ok(fs)
    }

    pub fn describe_file_systems(
        &self,
        file_system_id: Option<&str>,
        creation_token: Option<&str>,
    ) -> Result<Vec<&FileSystem>, EfsError> {
        if let Some(fs_id) = file_system_id {
            match self.file_systems.get(fs_id) {
                Some(fs) => Ok(vec![fs]),
                None => Err(EfsError::FileSystemNotFound(fs_id.to_string())),
            }
        } else if let Some(token) = creation_token {
            let result: Vec<&FileSystem> = self
                .file_systems
                .values()
                .filter(|fs| fs.creation_token == token)
                .collect();
            Ok(result)
        } else {
            Ok(self.file_systems.values().collect())
        }
    }

    pub fn delete_file_system(&mut self, file_system_id: &str) -> Result<(), EfsError> {
        // Check for mount targets
        let has_mount_targets = self
            .mount_targets
            .values()
            .any(|mt| mt.file_system_id == file_system_id);

        if has_mount_targets {
            return Err(EfsError::FileSystemInUse(file_system_id.to_string()));
        }

        match self.file_systems.remove(file_system_id) {
            Some(fs) => {
                self.creation_tokens.remove(&fs.creation_token);
                Ok(())
            }
            None => Err(EfsError::FileSystemNotFound(file_system_id.to_string())),
        }
    }

    pub fn create_mount_target(
        &mut self,
        file_system_id: &str,
        subnet_id: &str,
        owner_id: &str,
        ip_address: Option<&str>,
        security_group_ids: Option<&[String]>,
    ) -> Result<MountTarget, EfsError> {
        // Verify file system exists
        if !self.file_systems.contains_key(file_system_id) {
            return Err(EfsError::FileSystemNotFound(file_system_id.to_string()));
        }

        let mt_id = format!(
            "fsmt-{}",
            &uuid::Uuid::new_v4().to_string().replace('-', "")[..17]
        );
        let ip = ip_address.unwrap_or("10.0.0.1").to_string();
        let eni_id = format!(
            "eni-{}",
            &uuid::Uuid::new_v4().to_string().replace('-', "")[..17]
        );

        let security_groups = security_group_ids
            .map(|sgs| sgs.to_vec())
            .unwrap_or_else(|| {
                vec![format!(
                    "sg-{}",
                    &uuid::Uuid::new_v4().to_string().replace('-', "")[..17]
                )]
            });

        let mt = MountTarget {
            mount_target_id: mt_id.clone(),
            file_system_id: file_system_id.to_string(),
            subnet_id: subnet_id.to_string(),
            life_cycle_state: "available".to_string(),
            ip_address: ip,
            network_interface_id: eni_id,
            availability_zone_id: "use1-az1".to_string(),
            availability_zone_name: "us-east-1a".to_string(),
            owner_id: owner_id.to_string(),
            security_groups,
        };

        self.mount_targets.insert(mt_id.clone(), mt.clone());

        // Increment mount target count on the file system
        if let Some(fs) = self.file_systems.get_mut(file_system_id) {
            fs.number_of_mount_targets += 1;
        }

        Ok(mt)
    }

    pub fn describe_mount_targets(
        &self,
        file_system_id: Option<&str>,
        mount_target_id: Option<&str>,
    ) -> Result<Vec<&MountTarget>, EfsError> {
        if let Some(mt_id) = mount_target_id {
            match self.mount_targets.get(mt_id) {
                Some(mt) => Ok(vec![mt]),
                None => Err(EfsError::MountTargetNotFound(mt_id.to_string())),
            }
        } else if let Some(fs_id) = file_system_id {
            if !self.file_systems.contains_key(fs_id) {
                return Err(EfsError::FileSystemNotFound(fs_id.to_string()));
            }
            let mts: Vec<&MountTarget> = self
                .mount_targets
                .values()
                .filter(|mt| mt.file_system_id == fs_id)
                .collect();
            Ok(mts)
        } else {
            Err(EfsError::MissingMountTargetFilter)
        }
    }

    pub fn delete_mount_target(&mut self, mount_target_id: &str) -> Result<(), EfsError> {
        match self.mount_targets.remove(mount_target_id) {
            Some(mt) => {
                if let Some(fs) = self.file_systems.get_mut(&mt.file_system_id) {
                    fs.number_of_mount_targets -= 1;
                }
                Ok(())
            }
            None => Err(EfsError::MountTargetNotFound(mount_target_id.to_string())),
        }
    }

    pub fn create_access_point(
        &mut self,
        client_token: &str,
        file_system_id: &str,
        owner_id: &str,
        region: &str,
        posix_user: Option<PosixUser>,
        root_directory: Option<RootDirectory>,
        tags: Vec<Tag>,
    ) -> Result<AccessPoint, EfsError> {
        // Idempotency: scoped to (client_token, file_system_id)
        let token_key = format!("{client_token}:{file_system_id}");
        if let Some(ap_id) = self.access_point_tokens.get(&token_key) {
            if let Some(ap) = self.access_points.get(ap_id) {
                return Ok(ap.clone());
            }
        }

        // Verify file system exists
        if !self.file_systems.contains_key(file_system_id) {
            return Err(EfsError::FileSystemNotFound(file_system_id.to_string()));
        }

        let ap_id = format!(
            "fsap-{}",
            &uuid::Uuid::new_v4().to_string().replace('-', "")[..17]
        );
        let ap_arn = format!("arn:aws:elasticfilesystem:{region}:{owner_id}:access-point/{ap_id}");

        let name = tags
            .iter()
            .find(|t| t.key == "Name")
            .map(|t| t.value.clone());

        // Default root directory is {"Path": "/"} when none specified
        let root_directory = root_directory.unwrap_or_else(|| RootDirectory {
            path: Some("/".to_string()),
            creation_info: None,
        });

        let ap = AccessPoint {
            access_point_id: ap_id.clone(),
            access_point_arn: ap_arn,
            client_token: client_token.to_string(),
            file_system_id: file_system_id.to_string(),
            life_cycle_state: "available".to_string(),
            name,
            owner_id: owner_id.to_string(),
            posix_user,
            root_directory: Some(root_directory),
            tags,
        };

        self.access_point_tokens.insert(token_key, ap_id.clone());
        self.access_points.insert(ap_id, ap.clone());
        Ok(ap)
    }

    pub fn delete_access_point(&mut self, access_point_id: &str) -> Result<(), EfsError> {
        match self.access_points.remove(access_point_id) {
            Some(ap) => {
                let token_key = format!("{}:{}", ap.client_token, ap.file_system_id);
                self.access_point_tokens.remove(&token_key);
                Ok(())
            }
            None => Err(EfsError::AccessPointNotFound(access_point_id.to_string())),
        }
    }

    pub fn describe_access_points(
        &self,
        access_point_id: Option<&str>,
        file_system_id: Option<&str>,
    ) -> Result<Vec<&AccessPoint>, EfsError> {
        if let Some(ap_id) = access_point_id {
            match self.access_points.get(ap_id) {
                Some(ap) => Ok(vec![ap]),
                None => Err(EfsError::AccessPointNotFound(ap_id.to_string())),
            }
        } else if let Some(fs_id) = file_system_id {
            if !self.file_systems.contains_key(fs_id) {
                return Err(EfsError::FileSystemNotFound(fs_id.to_string()));
            }
            let aps: Vec<&AccessPoint> = self
                .access_points
                .values()
                .filter(|ap| ap.file_system_id == fs_id)
                .collect();
            Ok(aps)
        } else {
            Ok(self.access_points.values().collect())
        }
    }

    pub fn describe_mount_target_security_groups(
        &self,
        mount_target_id: &str,
    ) -> Result<Vec<String>, EfsError> {
        match self.mount_targets.get(mount_target_id) {
            Some(mt) => Ok(mt.security_groups.clone()),
            None => Err(EfsError::MountTargetNotFound(mount_target_id.to_string())),
        }
    }

    pub fn put_file_system_policy(
        &mut self,
        file_system_id: &str,
        policy: &str,
    ) -> Result<(String, String), EfsError> {
        match self.file_systems.get_mut(file_system_id) {
            Some(fs) => {
                fs.policy = Some(policy.to_string());
                Ok((file_system_id.to_string(), policy.to_string()))
            }
            None => Err(EfsError::FileSystemNotFound(file_system_id.to_string())),
        }
    }

    pub fn describe_file_system_policy(
        &self,
        file_system_id: &str,
    ) -> Result<(String, String), EfsError> {
        match self.file_systems.get(file_system_id) {
            Some(fs) => match &fs.policy {
                Some(policy) => Ok((file_system_id.to_string(), policy.clone())),
                None => Err(EfsError::PolicyNotFound(file_system_id.to_string())),
            },
            None => Err(EfsError::FileSystemNotFound(file_system_id.to_string())),
        }
    }

    pub fn delete_file_system_policy(&mut self, file_system_id: &str) -> Result<(), EfsError> {
        match self.file_systems.get_mut(file_system_id) {
            Some(fs) => {
                fs.policy = None;
                Ok(())
            }
            None => Err(EfsError::FileSystemNotFound(file_system_id.to_string())),
        }
    }

    pub fn list_tags_for_resource(&self, resource_id: &str) -> Result<Vec<Tag>, EfsError> {
        // Check file systems
        if let Some(fs) = self.file_systems.get(resource_id) {
            return Ok(fs.tags.clone());
        }
        // Check access points
        if let Some(ap) = self.access_points.get(resource_id) {
            return Ok(ap.tags.clone());
        }
        Err(EfsError::ResourceNotFound(resource_id.to_string()))
    }

    pub fn tag_resource(&mut self, resource_id: &str, tags: Vec<Tag>) -> Result<(), EfsError> {
        // Check file systems
        if let Some(fs) = self.file_systems.get_mut(resource_id) {
            for new_tag in &tags {
                if let Some(existing) = fs.tags.iter_mut().find(|t| t.key == new_tag.key) {
                    existing.value = new_tag.value.clone();
                } else {
                    fs.tags.push(new_tag.clone());
                }
                if new_tag.key == "Name" {
                    fs.name = Some(new_tag.value.clone());
                }
            }
            return Ok(());
        }
        // Check access points
        if let Some(ap) = self.access_points.get_mut(resource_id) {
            for new_tag in &tags {
                if let Some(existing) = ap.tags.iter_mut().find(|t| t.key == new_tag.key) {
                    existing.value = new_tag.value.clone();
                } else {
                    ap.tags.push(new_tag.clone());
                }
                if new_tag.key == "Name" {
                    ap.name = Some(new_tag.value.clone());
                }
            }
            return Ok(());
        }
        Err(EfsError::ResourceNotFound(resource_id.to_string()))
    }

    pub fn describe_lifecycle_configuration(
        &self,
        file_system_id: &str,
    ) -> Result<Vec<LifecyclePolicy>, EfsError> {
        match self.file_systems.get(file_system_id) {
            Some(fs) => Ok(fs.lifecycle_policies.clone()),
            None => Err(EfsError::FileSystemNotFound(file_system_id.to_string())),
        }
    }

    pub fn put_lifecycle_configuration(
        &mut self,
        file_system_id: &str,
        lifecycle_policies: Vec<LifecyclePolicy>,
    ) -> Result<Vec<LifecyclePolicy>, EfsError> {
        match self.file_systems.get_mut(file_system_id) {
            Some(fs) => {
                fs.lifecycle_policies = lifecycle_policies.clone();
                Ok(lifecycle_policies)
            }
            None => Err(EfsError::FileSystemNotFound(file_system_id.to_string())),
        }
    }

    pub fn modify_mount_target_security_groups(
        &mut self,
        mount_target_id: &str,
        security_groups: Vec<String>,
    ) -> Result<(), EfsError> {
        match self.mount_targets.get_mut(mount_target_id) {
            Some(mt) => {
                mt.security_groups = security_groups;
                Ok(())
            }
            None => Err(EfsError::MountTargetNotFound(mount_target_id.to_string())),
        }
    }

    pub fn untag_resource(
        &mut self,
        resource_id: &str,
        tag_keys: &[String],
    ) -> Result<(), EfsError> {
        // Check file systems
        if let Some(fs) = self.file_systems.get_mut(resource_id) {
            fs.tags.retain(|t| !tag_keys.contains(&t.key));
            if tag_keys.iter().any(|k| k == "Name") {
                fs.name = None;
            }
            return Ok(());
        }
        // Check access points
        if let Some(ap) = self.access_points.get_mut(resource_id) {
            ap.tags.retain(|t| !tag_keys.contains(&t.key));
            if tag_keys.iter().any(|k| k == "Name") {
                ap.name = None;
            }
            return Ok(());
        }
        Err(EfsError::ResourceNotFound(resource_id.to_string()))
    }

    // --- Backup policy operations ---

    pub fn describe_backup_policy(&self, fs_id: &str) -> Result<Option<String>, EfsError> {
        let fs = self
            .file_systems
            .get(fs_id)
            .ok_or_else(|| EfsError::FileSystemNotFound(fs_id.to_string()))?;
        Ok(fs.backup_policy_status.clone())
    }

    pub fn put_backup_policy(&mut self, fs_id: &str, status: &str) -> Result<String, EfsError> {
        let fs = self
            .file_systems
            .get_mut(fs_id)
            .ok_or_else(|| EfsError::FileSystemNotFound(fs_id.to_string()))?;
        fs.backup_policy_status = Some(status.to_string());
        Ok(status.to_string())
    }

    // --- Legacy tag operations ---

    pub fn create_tags(&mut self, file_system_id: &str, tags: Vec<Tag>) -> Result<(), EfsError> {
        let fs = self
            .file_systems
            .get_mut(file_system_id)
            .ok_or_else(|| EfsError::FileSystemNotFound(file_system_id.to_string()))?;
        for new_tag in &tags {
            if let Some(existing) = fs.tags.iter_mut().find(|t| t.key == new_tag.key) {
                existing.value = new_tag.value.clone();
            } else {
                fs.tags.push(new_tag.clone());
            }
            if new_tag.key == "Name" {
                fs.name = Some(new_tag.value.clone());
            }
        }
        Ok(())
    }

    pub fn delete_tags(
        &mut self,
        file_system_id: &str,
        tag_keys: &[String],
    ) -> Result<(), EfsError> {
        let fs = self
            .file_systems
            .get_mut(file_system_id)
            .ok_or_else(|| EfsError::FileSystemNotFound(file_system_id.to_string()))?;
        fs.tags.retain(|t| !tag_keys.contains(&t.key));
        if tag_keys.iter().any(|k| k == "Name") {
            fs.name = None;
        }
        Ok(())
    }

    pub fn describe_tags(&self, file_system_id: &str) -> Result<Vec<Tag>, EfsError> {
        let fs = self
            .file_systems
            .get(file_system_id)
            .ok_or_else(|| EfsError::FileSystemNotFound(file_system_id.to_string()))?;
        Ok(fs.tags.clone())
    }

    // --- Replication configuration operations ---

    pub fn create_replication_configuration(
        &mut self,
        source_file_system_id: &str,
        owner_id: &str,
        region: &str,
        destinations: Vec<ReplicationDestination>,
    ) -> Result<ReplicationConfiguration, EfsError> {
        if !self.file_systems.contains_key(source_file_system_id) {
            return Err(EfsError::FileSystemNotFound(
                source_file_system_id.to_string(),
            ));
        }
        if self
            .replication_configurations
            .contains_key(source_file_system_id)
        {
            return Err(EfsError::ReplicationAlreadyExists(
                source_file_system_id.to_string(),
            ));
        }
        let arn = format!(
            "arn:aws:elasticfilesystem:{region}:{owner_id}:file-system/{source_file_system_id}"
        );
        let rc = ReplicationConfiguration {
            source_file_system_id: source_file_system_id.to_string(),
            source_file_system_arn: arn.clone(),
            original_source_file_system_arn: arn,
            source_file_system_region: region.to_string(),
            source_file_system_owner_id: owner_id.to_string(),
            creation_time: Utc::now(),
            destinations,
        };
        self.replication_configurations
            .insert(source_file_system_id.to_string(), rc.clone());
        Ok(rc)
    }

    pub fn delete_replication_configuration(
        &mut self,
        source_file_system_id: &str,
    ) -> Result<(), EfsError> {
        match self
            .replication_configurations
            .remove(source_file_system_id)
        {
            Some(_) => Ok(()),
            None => Err(EfsError::ReplicationNotFound(
                source_file_system_id.to_string(),
            )),
        }
    }

    pub fn describe_replication_configurations(
        &self,
        file_system_id: Option<&str>,
    ) -> Result<Vec<&ReplicationConfiguration>, EfsError> {
        if let Some(fs_id) = file_system_id {
            match self.replication_configurations.get(fs_id) {
                Some(rc) => Ok(vec![rc]),
                None => Ok(vec![]),
            }
        } else {
            Ok(self.replication_configurations.values().collect())
        }
    }

    // --- Account preferences operations ---

    pub fn describe_account_preferences(&self) -> AccountPreferences {
        self.account_preferences
            .clone()
            .unwrap_or_else(|| AccountPreferences {
                resource_id_type: "LONG_ID".to_string(),
                resources: vec!["FILE_SYSTEM".to_string(), "MOUNT_TARGET".to_string()],
            })
    }

    pub fn put_account_preferences(
        &mut self,
        resource_id_type: &str,
        resources: Vec<String>,
    ) -> AccountPreferences {
        let prefs = AccountPreferences {
            resource_id_type: resource_id_type.to_string(),
            resources,
        };
        self.account_preferences = Some(prefs.clone());
        prefs
    }

    // --- UpdateFileSystem ---

    pub fn update_file_system(
        &mut self,
        file_system_id: &str,
        throughput_mode: Option<&str>,
        provisioned_throughput_in_mibps: Option<f64>,
    ) -> Result<FileSystem, EfsError> {
        let fs = self
            .file_systems
            .get_mut(file_system_id)
            .ok_or_else(|| EfsError::FileSystemNotFound(file_system_id.to_string()))?;
        if let Some(mode) = throughput_mode {
            fs.throughput_mode = mode.to_string();
        }
        let _ = provisioned_throughput_in_mibps; // stored on wire layer only for now
        Ok(fs.clone())
    }

    // --- UpdateFileSystemProtection ---

    pub fn update_file_system_protection(
        &mut self,
        file_system_id: &str,
        replication_overwrite_protection: Option<&str>,
    ) -> Result<FileSystemProtection, EfsError> {
        if !self.file_systems.contains_key(file_system_id) {
            return Err(EfsError::FileSystemNotFound(file_system_id.to_string()));
        }
        let protection = self
            .fs_protection
            .entry(file_system_id.to_string())
            .or_insert_with(|| FileSystemProtection {
                replication_overwrite_protection: None,
            });
        if let Some(val) = replication_overwrite_protection {
            protection.replication_overwrite_protection = Some(val.to_string());
        }
        Ok(protection.clone())
    }

    pub fn get_file_system_protection(&self, file_system_id: &str) -> FileSystemProtection {
        self.fs_protection
            .get(file_system_id)
            .cloned()
            .unwrap_or(FileSystemProtection {
                replication_overwrite_protection: None,
            })
    }
}

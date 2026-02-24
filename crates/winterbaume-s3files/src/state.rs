use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;
use uuid::Uuid;

use crate::types::{
    AccessPoint, ExpirationDataRule, FileSystem, ImportDataRule, MountTarget, PosixUser,
    RootDirectory, SynchronizationConfiguration,
};

#[derive(Debug, Default)]
pub struct S3FilesState {
    pub file_systems: HashMap<String, FileSystem>,
    pub mount_targets: HashMap<String, MountTarget>,
    pub access_points: HashMap<String, AccessPoint>,
}

#[derive(Debug, Error)]
pub enum S3FilesError {
    #[error("File system {0} does not exist.")]
    FileSystemNotFound(String),
    #[error("Mount target {0} does not exist.")]
    MountTargetNotFound(String),
    #[error("Access point {0} does not exist.")]
    AccessPointNotFound(String),
    #[error("Resource {0} does not exist.")]
    ResourceNotFound(String),
    #[error("File system {0} has no resource policy.")]
    PolicyNotFound(String),
    /// File system has mount targets in a different VPC; the documented constraint is
    /// "a file system can have mount targets in only one VPC at a time".
    #[error(
        "File system {file_system_id} already has mount targets in VPC {existing_vpc_id}; mount target requested in VPC {requested_vpc_id}."
    )]
    MountTargetVpcConflict {
        file_system_id: String,
        existing_vpc_id: String,
        requested_vpc_id: String,
    },
    /// File system already has a mount target in this AZ ( one per AZ per file system ).
    #[error(
        "File system {file_system_id} already has a mount target in availability zone {availability_zone_id}."
    )]
    MountTargetAzConflict {
        file_system_id: String,
        availability_zone_id: String,
    },
    /// Stale `latestVersionNumber` on `PutSynchronizationConfiguration`.
    #[error("Synchronization configuration version mismatch: expected {expected}, got {provided}.")]
    SynchronizationVersionConflict { expected: i32, provided: i32 },
    /// File system cannot be deleted while it still has mount targets or access points.
    #[error("Cannot delete file system {0}: {1} dependent resource(s) still attached.")]
    FileSystemInUse(String, usize),
    #[error("{message}")]
    Validation { message: String },
}

impl S3FilesState {
    #[allow(clippy::too_many_arguments)]
    pub fn create_file_system(
        &mut self,
        bucket: String,
        prefix: Option<String>,
        role_arn: String,
        kms_key_id: Option<String>,
        client_token: Option<String>,
        tags: HashMap<String, String>,
        owner_id: &str,
        region: &str,
    ) -> Result<&FileSystem, S3FilesError> {
        if bucket.is_empty() {
            return Err(S3FilesError::Validation {
                message: "bucket must not be empty".to_string(),
            });
        }
        if role_arn.is_empty() {
            return Err(S3FilesError::Validation {
                message: "roleArn must not be empty".to_string(),
            });
        }
        if let Some(token) = client_token.as_deref()
            && let Some(existing) = self
                .file_systems
                .values()
                .find(|fs| fs.client_token.as_deref() == Some(token))
        {
            let id = existing.file_system_id.clone();
            return Ok(self.file_systems.get(&id).unwrap());
        }

        let file_system_id = format!("fs-{}", Uuid::new_v4().simple());
        let file_system_arn = format!(
            "arn:aws:s3files:{}:{}:file-system/{}",
            region, owner_id, file_system_id
        );
        let fs = FileSystem {
            file_system_id: file_system_id.clone(),
            file_system_arn,
            bucket,
            prefix,
            kms_key_id,
            role_arn,
            client_token,
            name: None,
            status: "available".to_string(),
            status_message: None,
            creation_time: Utc::now(),
            owner_id: owner_id.to_string(),
            tags,
            policy: None,
            synchronization_configuration: SynchronizationConfiguration::default(),
        };
        self.file_systems.insert(file_system_id.clone(), fs);
        Ok(self.file_systems.get(&file_system_id).unwrap())
    }

    pub fn get_file_system(&self, file_system_id: &str) -> Result<&FileSystem, S3FilesError> {
        self.file_systems
            .get(file_system_id)
            .ok_or_else(|| S3FilesError::FileSystemNotFound(file_system_id.to_string()))
    }

    pub fn list_file_systems(&self, bucket_filter: Option<&str>) -> Vec<&FileSystem> {
        let mut entries: Vec<&FileSystem> = self
            .file_systems
            .values()
            .filter(|fs| match bucket_filter {
                Some(b) => fs.bucket == b,
                None => true,
            })
            .collect();
        entries.sort_by(|a, b| a.file_system_id.cmp(&b.file_system_id));
        entries
    }

    pub fn delete_file_system(
        &mut self,
        file_system_id: &str,
        _force_delete: bool,
    ) -> Result<(), S3FilesError> {
        if !self.file_systems.contains_key(file_system_id) {
            return Err(S3FilesError::FileSystemNotFound(file_system_id.to_string()));
        }
        let mt_count = self
            .mount_targets
            .values()
            .filter(|mt| mt.file_system_id == file_system_id)
            .count();
        let ap_count = self
            .access_points
            .values()
            .filter(|ap| ap.file_system_id == file_system_id)
            .count();
        if mt_count + ap_count > 0 {
            return Err(S3FilesError::FileSystemInUse(
                file_system_id.to_string(),
                mt_count + ap_count,
            ));
        }
        self.file_systems.remove(file_system_id);
        Ok(())
    }

    pub fn tag_resource(
        &mut self,
        resource_id: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), S3FilesError> {
        let fs = self
            .file_systems
            .get_mut(resource_id)
            .ok_or_else(|| S3FilesError::ResourceNotFound(resource_id.to_string()))?;
        for (k, v) in tags {
            fs.tags.insert(k, v);
        }
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_id: &str,
        keys: &[String],
    ) -> Result<(), S3FilesError> {
        let fs = self
            .file_systems
            .get_mut(resource_id)
            .ok_or_else(|| S3FilesError::ResourceNotFound(resource_id.to_string()))?;
        for k in keys {
            fs.tags.remove(k);
        }
        Ok(())
    }

    pub fn list_tags_for_resource(
        &self,
        resource_id: &str,
    ) -> Result<&HashMap<String, String>, S3FilesError> {
        self.file_systems
            .get(resource_id)
            .map(|fs| &fs.tags)
            .ok_or_else(|| S3FilesError::ResourceNotFound(resource_id.to_string()))
    }

    // --- Mount targets ----------------------------------------------------

    #[allow(clippy::too_many_arguments)]
    pub fn create_mount_target(
        &mut self,
        file_system_id: &str,
        subnet_id: String,
        ipv4_address: Option<String>,
        ipv6_address: Option<String>,
        ip_address_type: Option<String>,
        security_groups: Vec<String>,
        owner_id: &str,
    ) -> Result<&MountTarget, S3FilesError> {
        if !self.file_systems.contains_key(file_system_id) {
            return Err(S3FilesError::FileSystemNotFound(file_system_id.to_string()));
        }
        if subnet_id.is_empty() {
            return Err(S3FilesError::Validation {
                message: "subnetId must not be empty".to_string(),
            });
        }

        // Mocked subnet/AZ resolution: derive deterministic AZ + VPC from subnet id.
        let availability_zone_id = derive_az_from_subnet(&subnet_id);
        let vpc_id = derive_vpc_from_subnet(&subnet_id);

        // VPC constraint: file system can have mount targets in only one VPC.
        if let Some(existing_vpc) = self
            .mount_targets
            .values()
            .find(|mt| mt.file_system_id == file_system_id)
            .map(|mt| mt.vpc_id.clone())
            && existing_vpc != vpc_id
        {
            return Err(S3FilesError::MountTargetVpcConflict {
                file_system_id: file_system_id.to_string(),
                existing_vpc_id: existing_vpc,
                requested_vpc_id: vpc_id,
            });
        }
        // AZ constraint: at most one mount target per AZ per file system.
        if self.mount_targets.values().any(|mt| {
            mt.file_system_id == file_system_id && mt.availability_zone_id == availability_zone_id
        }) {
            return Err(S3FilesError::MountTargetAzConflict {
                file_system_id: file_system_id.to_string(),
                availability_zone_id,
            });
        }

        let mount_target_id = format!("mt-{}", Uuid::new_v4().simple());
        let network_interface_id = format!("eni-{}", Uuid::new_v4().simple());
        let mt = MountTarget {
            mount_target_id: mount_target_id.clone(),
            file_system_id: file_system_id.to_string(),
            subnet_id,
            vpc_id,
            availability_zone_id,
            ipv4_address,
            ipv6_address,
            ip_address_type: ip_address_type.unwrap_or_else(|| "IPV4_ONLY".to_string()),
            network_interface_id,
            security_groups,
            status: "available".to_string(),
            status_message: None,
            owner_id: owner_id.to_string(),
        };
        self.mount_targets.insert(mount_target_id.clone(), mt);
        Ok(self.mount_targets.get(&mount_target_id).unwrap())
    }

    pub fn get_mount_target(&self, mount_target_id: &str) -> Result<&MountTarget, S3FilesError> {
        self.mount_targets
            .get(mount_target_id)
            .ok_or_else(|| S3FilesError::MountTargetNotFound(mount_target_id.to_string()))
    }

    pub fn list_mount_targets(
        &self,
        file_system_id: Option<&str>,
        access_point_id: Option<&str>,
        vpc_id: Option<&str>,
    ) -> Vec<&MountTarget> {
        // If access_point_id is given, scope to its file system.
        let scoped_fs = access_point_id
            .and_then(|id| self.access_points.get(id))
            .map(|ap| ap.file_system_id.as_str());
        let fs_filter = file_system_id.or(scoped_fs);
        let mut entries: Vec<&MountTarget> = self
            .mount_targets
            .values()
            .filter(|mt| fs_filter.is_none_or(|f| mt.file_system_id == f))
            .filter(|mt| vpc_id.is_none_or(|v| mt.vpc_id == v))
            .collect();
        entries.sort_by(|a, b| a.mount_target_id.cmp(&b.mount_target_id));
        entries
    }

    pub fn delete_mount_target(&mut self, mount_target_id: &str) -> Result<(), S3FilesError> {
        self.mount_targets
            .remove(mount_target_id)
            .map(|_| ())
            .ok_or_else(|| S3FilesError::MountTargetNotFound(mount_target_id.to_string()))
    }

    pub fn update_mount_target(
        &mut self,
        mount_target_id: &str,
        security_groups: Vec<String>,
    ) -> Result<&MountTarget, S3FilesError> {
        let mt = self
            .mount_targets
            .get_mut(mount_target_id)
            .ok_or_else(|| S3FilesError::MountTargetNotFound(mount_target_id.to_string()))?;
        mt.security_groups = security_groups;
        Ok(mt)
    }

    // --- Access points ----------------------------------------------------

    #[allow(clippy::too_many_arguments)]
    pub fn create_access_point(
        &mut self,
        file_system_id: &str,
        posix_user: Option<PosixUser>,
        root_directory: Option<RootDirectory>,
        client_token: Option<String>,
        tags: HashMap<String, String>,
        owner_id: &str,
        region: &str,
    ) -> Result<&AccessPoint, S3FilesError> {
        if !self.file_systems.contains_key(file_system_id) {
            return Err(S3FilesError::FileSystemNotFound(file_system_id.to_string()));
        }
        if let Some(token) = client_token.as_deref()
            && let Some(existing) = self
                .access_points
                .values()
                .find(|ap| ap.client_token.as_deref() == Some(token))
        {
            let id = existing.access_point_id.clone();
            return Ok(self.access_points.get(&id).unwrap());
        }

        let access_point_id = format!("ap-{}", Uuid::new_v4().simple());
        let access_point_arn = format!(
            "arn:aws:s3files:{}:{}:access-point/{}",
            region, owner_id, access_point_id
        );
        let ap = AccessPoint {
            access_point_id: access_point_id.clone(),
            access_point_arn,
            file_system_id: file_system_id.to_string(),
            name: None,
            posix_user,
            root_directory,
            status: "available".to_string(),
            owner_id: owner_id.to_string(),
            client_token,
            tags,
        };
        self.access_points.insert(access_point_id.clone(), ap);
        Ok(self.access_points.get(&access_point_id).unwrap())
    }

    pub fn get_access_point(&self, access_point_id: &str) -> Result<&AccessPoint, S3FilesError> {
        self.access_points
            .get(access_point_id)
            .ok_or_else(|| S3FilesError::AccessPointNotFound(access_point_id.to_string()))
    }

    pub fn list_access_points(
        &self,
        file_system_id: &str,
    ) -> Result<Vec<&AccessPoint>, S3FilesError> {
        if !self.file_systems.contains_key(file_system_id) {
            return Err(S3FilesError::FileSystemNotFound(file_system_id.to_string()));
        }
        let mut entries: Vec<&AccessPoint> = self
            .access_points
            .values()
            .filter(|ap| ap.file_system_id == file_system_id)
            .collect();
        entries.sort_by(|a, b| a.access_point_id.cmp(&b.access_point_id));
        Ok(entries)
    }

    pub fn delete_access_point(&mut self, access_point_id: &str) -> Result<(), S3FilesError> {
        self.access_points
            .remove(access_point_id)
            .map(|_| ())
            .ok_or_else(|| S3FilesError::AccessPointNotFound(access_point_id.to_string()))
    }

    // --- File system policy ----------------------------------------------

    pub fn put_file_system_policy(
        &mut self,
        file_system_id: &str,
        policy: String,
    ) -> Result<(), S3FilesError> {
        if policy.len() > 20_000 {
            return Err(S3FilesError::Validation {
                message: "policy exceeds 20,000 character limit".to_string(),
            });
        }
        let fs = self
            .file_systems
            .get_mut(file_system_id)
            .ok_or_else(|| S3FilesError::FileSystemNotFound(file_system_id.to_string()))?;
        fs.policy = Some(policy);
        Ok(())
    }

    pub fn get_file_system_policy(&self, file_system_id: &str) -> Result<&str, S3FilesError> {
        let fs = self
            .file_systems
            .get(file_system_id)
            .ok_or_else(|| S3FilesError::FileSystemNotFound(file_system_id.to_string()))?;
        fs.policy
            .as_deref()
            .ok_or_else(|| S3FilesError::PolicyNotFound(file_system_id.to_string()))
    }

    pub fn delete_file_system_policy(&mut self, file_system_id: &str) -> Result<(), S3FilesError> {
        let fs = self
            .file_systems
            .get_mut(file_system_id)
            .ok_or_else(|| S3FilesError::FileSystemNotFound(file_system_id.to_string()))?;
        fs.policy = None;
        Ok(())
    }

    // --- Synchronization configuration -----------------------------------

    pub fn put_synchronization_configuration(
        &mut self,
        file_system_id: &str,
        latest_version_number: Option<i32>,
        import_data_rules: Vec<ImportDataRule>,
        expiration_data_rules: Vec<ExpirationDataRule>,
    ) -> Result<&SynchronizationConfiguration, S3FilesError> {
        if import_data_rules.len() > 10 {
            return Err(S3FilesError::Validation {
                message: "importDataRules accepts at most 10 rules".to_string(),
            });
        }
        let fs = self
            .file_systems
            .get_mut(file_system_id)
            .ok_or_else(|| S3FilesError::FileSystemNotFound(file_system_id.to_string()))?;
        let current = fs.synchronization_configuration.latest_version_number;
        if let Some(provided) = latest_version_number
            && provided != current
        {
            return Err(S3FilesError::SynchronizationVersionConflict {
                expected: current,
                provided,
            });
        }
        fs.synchronization_configuration = SynchronizationConfiguration {
            latest_version_number: current + 1,
            import_data_rules,
            expiration_data_rules,
        };
        Ok(&fs.synchronization_configuration)
    }

    pub fn get_synchronization_configuration(
        &self,
        file_system_id: &str,
    ) -> Result<&SynchronizationConfiguration, S3FilesError> {
        let fs = self
            .file_systems
            .get(file_system_id)
            .ok_or_else(|| S3FilesError::FileSystemNotFound(file_system_id.to_string()))?;
        Ok(&fs.synchronization_configuration)
    }
}

/// Mock helper: derive a stable AZ id from a subnet id. Real AWS resolves the AZ from
/// the subnet record; tests that need different AZs should use distinct subnet ids.
fn derive_az_from_subnet(subnet_id: &str) -> String {
    let suffix = subnet_id
        .bytes()
        .fold(0u32, |a, b| a.wrapping_add(b as u32))
        % 6;
    format!("use1-az{}", suffix + 1)
}

/// Mock helper: derive a stable VPC id from a subnet id by stripping the trailing
/// numeric suffix. `subnet-vpcA-az1` and `subnet-vpcA-az2` resolve to the same VPC,
/// `subnet-vpcA-az1` and `subnet-vpcB-az1` to different VPCs.
fn derive_vpc_from_subnet(subnet_id: &str) -> String {
    // Pull the second segment if subnet id is `subnet-<vpc-tag>-<az-tag>`; otherwise
    // hash the whole id so distinct subnets map to distinct VPCs.
    let parts: Vec<&str> = subnet_id.splitn(3, '-').collect();
    if parts.len() == 3 {
        format!("vpc-{}", parts[1])
    } else {
        format!("vpc-{:08x}", crc32(subnet_id))
    }
}

fn crc32(s: &str) -> u32 {
    s.bytes().fold(0xffffffff_u32, |acc, b| {
        acc.wrapping_mul(31).wrapping_add(b as u32)
    })
}

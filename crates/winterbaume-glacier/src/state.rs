use std::collections::HashMap;

use chrono::{TimeDelta, Utc};
use uuid;

use crate::types::*;

#[derive(Debug, Default)]
pub struct GlacierState {
    pub vaults: HashMap<String, Vault>,
    pub data_retrieval_policy: Option<DataRetrievalPolicy>,
    pub provisioned_capacity: Vec<ProvisionedCapacity>,
}

#[derive(Debug, thiserror::Error)]
pub enum GlacierError {
    #[error("Vault not found")]
    VaultNotFound,
    #[error("Vault not found")]
    ResourceNotFound,
    #[error("Job not found")]
    JobNotFound,
    #[error("Lock not found or lock ID mismatch")]
    LockNotFound,
    #[error("Multipart upload not found")]
    MultipartUploadNotFound,
}

impl GlacierState {
    pub fn create_vault(&mut self, vault_name: &str, account_id: &str, region: &str) {
        let arn = format!("arn:aws:glacier:{region}:{account_id}:vaults/{vault_name}");
        let vault = Vault {
            vault_name: vault_name.to_string(),
            arn,
            created_at: Utc::now(),
            archives: HashMap::new(),
            jobs: HashMap::new(),
            tags: HashMap::new(),
            access_policy: None,
            notification_config: None,
            vault_lock: None,
            multipart_uploads: HashMap::new(),
        };
        self.vaults.insert(vault_name.to_string(), vault);
    }

    pub fn describe_vault(&self, vault_name: &str) -> Result<&Vault, GlacierError> {
        self.vaults
            .get(vault_name)
            .ok_or(GlacierError::VaultNotFound)
    }

    pub fn list_vaults(&self) -> Vec<&Vault> {
        self.vaults.values().collect()
    }

    pub fn delete_vault(&mut self, vault_name: &str) {
        self.vaults.remove(vault_name);
    }

    /// Store an archive with a pre-computed blob key, sha256, and size.
    ///
    /// The caller is responsible for writing the body to the BlobStore under
    /// `blob_key` before calling this method.
    pub fn upload_archive(
        &mut self,
        vault_name: &str,
        archive_id: String,
        sha256: String,
        size: usize,
        blob_key: String,
        description: &str,
    ) -> Result<(), GlacierError> {
        let vault = self
            .vaults
            .get_mut(vault_name)
            .ok_or(GlacierError::VaultNotFound)?;

        let archive = Archive {
            archive_id: archive_id.clone(),
            blob_key,
            size,
            sha256,
            description: description.to_string(),
            creation_date: Utc::now(),
        };

        vault.archives.insert(archive_id, archive);
        Ok(())
    }

    /// Delete an archive and return its blob_key so the caller can clean up
    /// the blob from the BlobStore.
    pub fn delete_archive(
        &mut self,
        vault_name: &str,
        archive_id: &str,
    ) -> Result<Option<String>, GlacierError> {
        let vault = self
            .vaults
            .get_mut(vault_name)
            .ok_or(GlacierError::VaultNotFound)?;
        let blob_key = vault.archives.remove(archive_id).map(|a| a.blob_key);
        Ok(blob_key)
    }

    pub fn initiate_job(
        &mut self,
        vault_name: &str,
        job_type_str: &str,
        tier: &str,
        archive_id: Option<&str>,
    ) -> Result<String, GlacierError> {
        let vault = self
            .vaults
            .get_mut(vault_name)
            .ok_or(GlacierError::VaultNotFound)?;

        let job_id = generate_job_id();
        let now = Utc::now();

        let duration = match tier.to_lowercase().as_str() {
            "expedited" => TimeDelta::seconds(2),
            "bulk" => TimeDelta::seconds(10),
            _ => TimeDelta::seconds(5), // Standard
        };

        let job_type = if job_type_str == "inventory-retrieval" {
            JobType::InventoryRetrieval
        } else {
            JobType::ArchiveRetrieval
        };

        let job = Job {
            job_id: job_id.clone(),
            job_type,
            tier: tier.to_string(),
            vault_arn: vault.arn.clone(),
            archive_id: archive_id.map(|s| s.to_string()),
            created_at: now,
            completed_at: now + duration,
        };

        vault.jobs.insert(job_id.clone(), job);
        Ok(job_id)
    }

    pub fn describe_job(&self, vault_name: &str, job_id: &str) -> Result<&Job, GlacierError> {
        let vault = self.describe_vault(vault_name)?;
        vault.jobs.get(job_id).ok_or(GlacierError::JobNotFound)
    }

    pub fn list_jobs(&self, vault_name: &str) -> Result<Vec<&Job>, GlacierError> {
        let vault = self.describe_vault(vault_name)?;
        Ok(vault.jobs.values().collect())
    }

    // --- Tags ---

    pub fn add_tags_to_vault(
        &mut self,
        vault_name: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), GlacierError> {
        let vault = self
            .vaults
            .get_mut(vault_name)
            .ok_or(GlacierError::ResourceNotFound)?;
        for (k, v) in tags {
            vault.tags.insert(k, v);
        }
        Ok(())
    }

    pub fn list_tags_for_vault(
        &self,
        vault_name: &str,
    ) -> Result<HashMap<String, String>, GlacierError> {
        let vault = self
            .vaults
            .get(vault_name)
            .ok_or(GlacierError::ResourceNotFound)?;
        Ok(vault.tags.clone())
    }

    pub fn remove_tags_from_vault(
        &mut self,
        vault_name: &str,
        tag_keys: &[String],
    ) -> Result<(), GlacierError> {
        let vault = self
            .vaults
            .get_mut(vault_name)
            .ok_or(GlacierError::ResourceNotFound)?;
        for k in tag_keys {
            vault.tags.remove(k);
        }
        Ok(())
    }

    // --- Vault Access Policy ---

    pub fn set_vault_access_policy(
        &mut self,
        vault_name: &str,
        policy: Option<String>,
    ) -> Result<(), GlacierError> {
        let vault = self
            .vaults
            .get_mut(vault_name)
            .ok_or(GlacierError::ResourceNotFound)?;
        vault.access_policy = policy;
        Ok(())
    }

    pub fn get_vault_access_policy(
        &self,
        vault_name: &str,
    ) -> Result<Option<String>, GlacierError> {
        let vault = self
            .vaults
            .get(vault_name)
            .ok_or(GlacierError::ResourceNotFound)?;
        Ok(vault.access_policy.clone())
    }

    pub fn delete_vault_access_policy(&mut self, vault_name: &str) -> Result<(), GlacierError> {
        let vault = self
            .vaults
            .get_mut(vault_name)
            .ok_or(GlacierError::ResourceNotFound)?;
        vault.access_policy = None;
        Ok(())
    }

    // --- Vault Notifications ---

    pub fn set_vault_notifications(
        &mut self,
        vault_name: &str,
        sns_topic: Option<String>,
        events: Vec<String>,
    ) -> Result<(), GlacierError> {
        let vault = self
            .vaults
            .get_mut(vault_name)
            .ok_or(GlacierError::ResourceNotFound)?;
        vault.notification_config = Some(VaultNotificationConfig { sns_topic, events });
        Ok(())
    }

    pub fn get_vault_notifications(
        &self,
        vault_name: &str,
    ) -> Result<Option<VaultNotificationConfig>, GlacierError> {
        let vault = self
            .vaults
            .get(vault_name)
            .ok_or(GlacierError::ResourceNotFound)?;
        Ok(vault.notification_config.clone())
    }

    pub fn delete_vault_notifications(&mut self, vault_name: &str) -> Result<(), GlacierError> {
        let vault = self
            .vaults
            .get_mut(vault_name)
            .ok_or(GlacierError::ResourceNotFound)?;
        vault.notification_config = None;
        Ok(())
    }

    // --- Vault Lock ---

    pub fn initiate_vault_lock(
        &mut self,
        vault_name: &str,
        policy: Option<String>,
    ) -> Result<String, GlacierError> {
        let vault = self
            .vaults
            .get_mut(vault_name)
            .ok_or(GlacierError::ResourceNotFound)?;
        let lock_id = uuid::Uuid::new_v4().to_string();
        vault.vault_lock = Some(VaultLock {
            lock_id: lock_id.clone(),
            policy,
            state: VaultLockState::InProgress,
            creation_date: Utc::now(),
        });
        Ok(lock_id)
    }

    pub fn complete_vault_lock(
        &mut self,
        vault_name: &str,
        lock_id: &str,
    ) -> Result<(), GlacierError> {
        let vault = self
            .vaults
            .get_mut(vault_name)
            .ok_or(GlacierError::ResourceNotFound)?;
        match &mut vault.vault_lock {
            Some(lock) if lock.lock_id == lock_id => {
                lock.state = VaultLockState::Locked;
                Ok(())
            }
            _ => Err(GlacierError::LockNotFound),
        }
    }

    pub fn abort_vault_lock(&mut self, vault_name: &str) -> Result<(), GlacierError> {
        let vault = self
            .vaults
            .get_mut(vault_name)
            .ok_or(GlacierError::ResourceNotFound)?;
        vault.vault_lock = None;
        Ok(())
    }

    pub fn get_vault_lock(&self, vault_name: &str) -> Result<Option<&VaultLock>, GlacierError> {
        let vault = self
            .vaults
            .get(vault_name)
            .ok_or(GlacierError::ResourceNotFound)?;
        Ok(vault.vault_lock.as_ref())
    }

    // --- Multipart Uploads ---

    pub fn initiate_multipart_upload(
        &mut self,
        vault_name: &str,
        vault_arn: &str,
        description: Option<String>,
        part_size: Option<i64>,
    ) -> Result<String, GlacierError> {
        let _vault = self
            .vaults
            .get(vault_name)
            .ok_or(GlacierError::ResourceNotFound)?;
        let upload_id = uuid::Uuid::new_v4().to_string();
        let vault = self.vaults.get_mut(vault_name).unwrap();
        vault.multipart_uploads.insert(
            upload_id.clone(),
            MultipartUpload {
                upload_id: upload_id.clone(),
                vault_arn: vault_arn.to_string(),
                description,
                part_size,
                creation_date: Utc::now(),
                parts: HashMap::new(),
            },
        );
        Ok(upload_id)
    }

    pub fn abort_multipart_upload(
        &mut self,
        vault_name: &str,
        upload_id: &str,
    ) -> Result<(), GlacierError> {
        let vault = self
            .vaults
            .get_mut(vault_name)
            .ok_or(GlacierError::ResourceNotFound)?;
        vault.multipart_uploads.remove(upload_id);
        Ok(())
    }

    pub fn upload_multipart_part(
        &mut self,
        vault_name: &str,
        upload_id: &str,
        range: Option<String>,
        sha256: String,
    ) -> Result<(), GlacierError> {
        let vault = self
            .vaults
            .get_mut(vault_name)
            .ok_or(GlacierError::ResourceNotFound)?;
        let upload = vault
            .multipart_uploads
            .get_mut(upload_id)
            .ok_or(GlacierError::MultipartUploadNotFound)?;
        let range_str = range.unwrap_or_default();
        upload.parts.insert(
            range_str.clone(),
            MultipartPart {
                range_in_bytes: range_str,
                sha256,
            },
        );
        Ok(())
    }

    pub fn complete_multipart_upload(
        &mut self,
        vault_name: &str,
        upload_id: &str,
        archive_size: Option<i64>,
        checksum: Option<String>,
    ) -> Result<String, GlacierError> {
        let vault = self
            .vaults
            .get_mut(vault_name)
            .ok_or(GlacierError::ResourceNotFound)?;
        let upload = vault
            .multipart_uploads
            .remove(upload_id)
            .ok_or(GlacierError::MultipartUploadNotFound)?;
        let archive_id = uuid::Uuid::new_v4().to_string();
        let archive = Archive {
            archive_id: archive_id.clone(),
            blob_key: String::new(), // no blob for multipart in mock
            size: archive_size.unwrap_or(0) as usize,
            sha256: checksum.unwrap_or_default(),
            description: upload.description.unwrap_or_default(),
            creation_date: Utc::now(),
        };
        vault.archives.insert(archive_id.clone(), archive);
        Ok(archive_id)
    }

    pub fn list_multipart_uploads(
        &self,
        vault_name: &str,
    ) -> Result<Vec<&MultipartUpload>, GlacierError> {
        let vault = self
            .vaults
            .get(vault_name)
            .ok_or(GlacierError::ResourceNotFound)?;
        Ok(vault.multipart_uploads.values().collect())
    }

    pub fn list_parts(
        &self,
        vault_name: &str,
        upload_id: &str,
    ) -> Result<&MultipartUpload, GlacierError> {
        let vault = self
            .vaults
            .get(vault_name)
            .ok_or(GlacierError::ResourceNotFound)?;
        vault
            .multipart_uploads
            .get(upload_id)
            .ok_or(GlacierError::MultipartUploadNotFound)
    }

    // --- Data Retrieval Policy ---

    pub fn set_data_retrieval_policy(&mut self, policy: Option<DataRetrievalPolicy>) {
        self.data_retrieval_policy = policy;
    }

    pub fn get_data_retrieval_policy(&self) -> Option<&DataRetrievalPolicy> {
        self.data_retrieval_policy.as_ref()
    }

    // --- Provisioned Capacity ---

    pub fn purchase_provisioned_capacity(&mut self) -> String {
        let capacity_id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        self.provisioned_capacity.push(ProvisionedCapacity {
            capacity_id: capacity_id.clone(),
            start_date: now,
            expiration_date: now + TimeDelta::days(365),
        });
        capacity_id
    }

    pub fn list_provisioned_capacity(&self) -> &[ProvisionedCapacity] {
        &self.provisioned_capacity
    }

    pub fn get_job_output(
        &self,
        vault_name: &str,
        job_id: &str,
    ) -> Result<Option<JobOutput<'_>>, GlacierError> {
        let vault = self.describe_vault(vault_name)?;
        let job = vault.jobs.get(job_id).ok_or(GlacierError::JobNotFound)?;

        if !job.is_completed() {
            return Ok(None);
        }

        match job.job_type {
            JobType::ArchiveRetrieval => {
                if let Some(ref archive_id) = job.archive_id
                    && let Some(archive) = vault.archives.get(archive_id)
                {
                    return Ok(Some(JobOutput::ArchiveBody(archive.blob_key.clone())));
                }
                Ok(None)
            }
            JobType::InventoryRetrieval => {
                let archives: Vec<_> = vault.archives.values().collect();
                Ok(Some(JobOutput::Inventory {
                    vault_arn: vault.arn.clone(),
                    archives,
                }))
            }
        }
    }
}

pub enum JobOutput<'a> {
    /// blob_key to look up in the service BlobStore
    ArchiveBody(String),
    Inventory {
        vault_arn: String,
        archives: Vec<&'a Archive>,
    },
}

fn generate_job_id() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::rng();
    (0..92)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

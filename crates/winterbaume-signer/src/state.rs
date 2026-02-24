use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

use thiserror::Error;

use crate::types::{ProfilePermission, SigningJob, SigningProfile};

#[derive(Debug, Default)]
pub struct SignerState {
    pub profiles: HashMap<String, SigningProfile>,
    pub jobs: HashMap<String, SigningJob>,
}

#[derive(Debug, Error)]
pub enum SignerError {
    #[error("Profile name must not be empty.")]
    ProfileNameEmpty,

    #[error("Profile version {given} does not match current version {current}.")]
    ProfileVersionMismatch { given: String, current: String },

    #[error("Signing profile {0} is not active.")]
    ProfileNotActive(String),

    #[error("A signing profile with name {0} does not exist.")]
    ProfileNotFound(String),

    #[error("Resource {0} not found.")]
    ResourceNotFound(String),

    #[error("Permission statement {0} not found.")]
    PermissionStatementNotFound(String),

    #[error("Signing job {0} not found.")]
    SigningJobNotFound(String),
}

impl SignerState {
    pub fn put_signing_profile(
        &mut self,
        profile_name: &str,
        platform_id: &str,
        region: &str,
        account_id: &str,
    ) -> Result<&SigningProfile, SignerError> {
        if profile_name.is_empty() {
            return Err(SignerError::ProfileNameEmpty);
        }

        let version = format!("{}", uuid::Uuid::new_v4());
        let arn = format!("arn:aws:signer:{region}:{account_id}:/signing-profiles/{profile_name}");
        let profile_version_arn = format!("{arn}/{version}");

        let profile = SigningProfile {
            profile_name: profile_name.to_string(),
            profile_version: version,
            profile_version_arn,
            platform_id: platform_id.to_string(),
            status: "Active".to_string(),
            arn,
            tags: HashMap::new(),
            permissions: Vec::new(),
            revision_id: format!("{}", uuid::Uuid::new_v4()),
            signature_validity_period: None,
            signing_material_certificate_arn: None,
        };

        self.profiles.insert(profile_name.to_string(), profile);
        Ok(self.profiles.get(profile_name).unwrap())
    }

    pub fn get_signing_profile(&self, profile_name: &str) -> Result<&SigningProfile, SignerError> {
        self.profiles
            .get(profile_name)
            .ok_or_else(|| SignerError::ProfileNotFound(profile_name.to_string()))
    }

    pub fn cancel_signing_profile(&mut self, profile_name: &str) -> Result<(), SignerError> {
        let profile = self
            .profiles
            .get_mut(profile_name)
            .ok_or_else(|| SignerError::ProfileNotFound(profile_name.to_string()))?;
        profile.status = "Canceled".to_string();
        Ok(())
    }

    pub fn list_signing_profiles(&self, include_canceled: bool) -> Vec<&SigningProfile> {
        self.profiles
            .values()
            .filter(|p| include_canceled || p.status == "Active")
            .collect()
    }

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), SignerError> {
        let profile = self
            .profiles
            .values_mut()
            .find(|p| p.arn == resource_arn)
            .ok_or_else(|| SignerError::ResourceNotFound(resource_arn.to_string()))?;
        profile.tags.extend(tags);
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), SignerError> {
        let profile = self
            .profiles
            .values_mut()
            .find(|p| p.arn == resource_arn)
            .ok_or_else(|| SignerError::ResourceNotFound(resource_arn.to_string()))?;
        for key in tag_keys {
            profile.tags.remove(key);
        }
        Ok(())
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<&HashMap<String, String>, SignerError> {
        let profile = self
            .profiles
            .values()
            .find(|p| p.arn == resource_arn)
            .ok_or_else(|| SignerError::ResourceNotFound(resource_arn.to_string()))?;
        Ok(&profile.tags)
    }

    pub fn add_profile_permission(
        &mut self,
        profile_name: &str,
        statement_id: &str,
        action: &str,
        principal: &str,
        profile_version: Option<String>,
    ) -> Result<String, SignerError> {
        let profile = self
            .profiles
            .get_mut(profile_name)
            .ok_or_else(|| SignerError::ProfileNotFound(profile_name.to_string()))?;
        // Remove any existing permission with same statementId
        profile
            .permissions
            .retain(|p| p.statement_id != statement_id);
        profile.permissions.push(ProfilePermission {
            statement_id: statement_id.to_string(),
            action: action.to_string(),
            principal: principal.to_string(),
            profile_version,
        });
        profile.revision_id = format!("{}", uuid::Uuid::new_v4());
        Ok(profile.revision_id.clone())
    }

    pub fn list_profile_permissions(
        &self,
        profile_name: &str,
    ) -> Result<(&Vec<ProfilePermission>, &str), SignerError> {
        let profile = self
            .profiles
            .get(profile_name)
            .ok_or_else(|| SignerError::ProfileNotFound(profile_name.to_string()))?;
        Ok((&profile.permissions, &profile.revision_id))
    }

    pub fn remove_profile_permission(
        &mut self,
        profile_name: &str,
        statement_id: &str,
    ) -> Result<String, SignerError> {
        let profile = self
            .profiles
            .get_mut(profile_name)
            .ok_or_else(|| SignerError::ProfileNotFound(profile_name.to_string()))?;
        let before = profile.permissions.len();
        profile
            .permissions
            .retain(|p| p.statement_id != statement_id);
        if profile.permissions.len() == before {
            return Err(SignerError::PermissionStatementNotFound(
                statement_id.to_string(),
            ));
        }
        profile.revision_id = format!("{}", uuid::Uuid::new_v4());
        Ok(profile.revision_id.clone())
    }

    pub fn revoke_signing_profile(
        &mut self,
        profile_name: &str,
        profile_version: &str,
    ) -> Result<(), SignerError> {
        let profile = self
            .profiles
            .get_mut(profile_name)
            .ok_or_else(|| SignerError::ProfileNotFound(profile_name.to_string()))?;
        if profile.profile_version != profile_version {
            return Err(SignerError::ProfileVersionMismatch {
                given: profile_version.to_string(),
                current: profile.profile_version.clone(),
            });
        }
        profile.status = "Revoked".to_string();
        Ok(())
    }

    pub fn start_signing_job(
        &mut self,
        profile_name: &str,
        account_id: &str,
        region: &str,
    ) -> Result<(String, String), SignerError> {
        let profile = self
            .profiles
            .get(profile_name)
            .ok_or_else(|| SignerError::ProfileNotFound(profile_name.to_string()))?;
        if profile.status != "Active" {
            return Err(SignerError::ProfileNotActive(profile_name.to_string()));
        }
        let job_id = format!("{}", uuid::Uuid::new_v4());
        let job_arn = format!("arn:aws:signer:{region}:{account_id}:/signing-jobs/{job_id}");
        let job = SigningJob {
            job_id: job_id.clone(),
            job_arn: job_arn.clone(),
            job_owner: account_id.to_string(),
            profile_name: profile.profile_name.clone(),
            profile_version: profile.profile_version.clone(),
            platform_id: profile.platform_id.clone(),
            status: "Succeeded".to_string(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs_f64(),
            is_revoked: false,
            revocation_reason: None,
            revoked_at: None,
        };
        self.jobs.insert(job_id.clone(), job);
        Ok((job_id, job_arn))
    }

    pub fn describe_signing_job(&self, job_id: &str) -> Result<&SigningJob, SignerError> {
        self.jobs
            .get(job_id)
            .ok_or_else(|| SignerError::SigningJobNotFound(job_id.to_string()))
    }

    pub fn list_signing_jobs(&self) -> Vec<&SigningJob> {
        self.jobs.values().collect()
    }

    pub fn revoke_signature(&mut self, job_id: &str) -> Result<(), SignerError> {
        let job = self
            .jobs
            .get_mut(job_id)
            .ok_or_else(|| SignerError::SigningJobNotFound(job_id.to_string()))?;
        job.status = "Revoked".to_string();
        job.is_revoked = true;
        job.revoked_at = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs_f64(),
        );
        Ok(())
    }

    pub fn sign_payload(
        &mut self,
        profile_name: &str,
        payload: &[u8],
        account_id: &str,
        region: &str,
    ) -> Result<(String, String, String), SignerError> {
        let (job_id, job_owner) = self.start_signing_job(profile_name, account_id, region)?;
        let signature = mock_signature(profile_name, payload);
        Ok((job_id, job_owner, signature))
    }
}

/// Produce a deterministic mock signature from profile name and payload.
///
/// Uses `DefaultHasher` in multiple rounds to build a 32-byte value, then
/// base64-encodes the result.  The same (profile_name, payload) pair always
/// yields the same signature; different inputs yield different signatures.
fn mock_signature(profile_name: &str, payload: &[u8]) -> String {
    use base64::Engine;

    let mut bytes = Vec::with_capacity(32);
    for i in 0u64..4 {
        let mut hasher = DefaultHasher::new();
        hasher.write(profile_name.as_bytes());
        hasher.write(payload);
        hasher.write_u64(i);
        bytes.extend_from_slice(&hasher.finish().to_le_bytes());
    }
    base64::engine::general_purpose::STANDARD.encode(&bytes)
}

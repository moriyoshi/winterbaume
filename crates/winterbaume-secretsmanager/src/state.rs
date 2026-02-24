use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct SecretsManagerState {
    pub secrets: HashMap<String, Secret>,
}

#[derive(Debug, thiserror::Error)]
pub enum SecretsManagerError {
    #[error("A resource with the ID you requested already exists.")]
    ResourceExists,
    #[error("You tried to perform the operation on a secret that's currently marked deleted.")]
    SecretMarkedDeleted,
    #[error("Secrets Manager can't find the specified secret value for VersionId: {version_id}")]
    SecretVersionNotFoundById { version_id: String },
    #[error("Secrets Manager can't find the specified secret value for staging label: {stage}")]
    SecretVersionNotFoundByStage { stage: String },
    #[error(
        "An error occurred (InvalidParameterException) when calling the DeleteSecret operation: RecoveryWindowInDays value must be between 7 and 30 days (inclusive)."
    )]
    InvalidRecoveryWindow,
    #[error(
        "An error occurred (InvalidParameterException) when calling the DeleteSecret operation: You can't use ForceDeleteWithoutRecovery in conjunction with RecoveryWindowInDays."
    )]
    ForceDeleteWithRecoveryWindow,
    #[error("Secrets Manager can't find the specified secret.")]
    SecretNotFound,
    #[error("ClientError")]
    PasswordTooShort,
    #[error("InvalidParameterValue")]
    PasswordTooLong,
    #[error("InvalidParameterException")]
    NoValidCharsForPassword,
    #[error("RotationRules.AutomaticallyAfterDays must be within 1-1000.")]
    InvalidRotationDays,
    #[error(
        "The parameter RemoveFromVersionId can't be empty. Staging label AWSCURRENT is currently attached to version {vid}, so you must explicitly reference that version in RemoveFromVersionId."
    )]
    MissingRemoveFromVersionId { vid: String },
    #[error("Not a valid version: {version_id}")]
    NotAValidVersion { version_id: String },
}

impl SecretsManagerState {
    pub fn create_secret(
        &mut self,
        name: &str,
        description: &str,
        secret_string: Option<&str>,
        secret_binary: Option<Vec<u8>>,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&Secret, SecretsManagerError> {
        if self.secrets.contains_key(name) {
            return Err(SecretsManagerError::ResourceExists);
        }

        let arn = format!(
            "arn:aws:secretsmanager:{region}:{account_id}:secret:{name}-{suffix}",
            suffix = &uuid::Uuid::new_v4().to_string()[..6]
        );
        let now = Utc::now();

        let has_value = secret_string.is_some() || secret_binary.is_some();

        let mut versions = HashMap::new();
        let current_version_id = if has_value {
            let version_id = uuid::Uuid::new_v4().to_string();
            let version = SecretVersion {
                version_id: version_id.clone(),
                secret_string: secret_string.map(|s| s.to_string()),
                secret_binary,
                created_date: now,
                version_stages: vec!["AWSCURRENT".to_string()],
            };
            versions.insert(version_id.clone(), version);
            Some(version_id)
        } else {
            None
        };

        let secret = Secret {
            name: name.to_string(),
            arn,
            description: description.to_string(),
            created_date: now,
            last_changed_date: now,
            versions,
            current_version_id,
            deleted_date: None,
            tags,
            resource_policy: None,
            rotation_enabled: None,
            rotation_lambda_arn: None,
            rotation_rules: None,
            last_rotated_date: None,
            replication_status: Vec::new(),
            primary_region: Some(region.to_string()),
        };

        self.secrets.insert(name.to_string(), secret);
        Ok(self.secrets.get(name).unwrap())
    }

    pub fn get_secret_value(
        &self,
        secret_id: &str,
    ) -> Result<(&Secret, &SecretVersion), SecretsManagerError> {
        self.get_secret_value_by_stage(secret_id, None)
    }

    pub fn get_secret_value_by_version_id(
        &self,
        secret_id: &str,
        version_id: &str,
    ) -> Result<(&Secret, &SecretVersion), SecretsManagerError> {
        let secret = self.find_secret(secret_id)?;

        if secret.deleted_date.is_some() {
            return Err(SecretsManagerError::SecretMarkedDeleted);
        }

        match secret.versions.get(version_id) {
            Some(version) => Ok((secret, version)),
            None => Err(SecretsManagerError::SecretVersionNotFoundById {
                version_id: version_id.to_string(),
            }),
        }
    }

    pub fn get_secret_value_by_stage(
        &self,
        secret_id: &str,
        version_stage: Option<&str>,
    ) -> Result<(&Secret, &SecretVersion), SecretsManagerError> {
        let secret = self.find_secret(secret_id)?;
        let stage = version_stage.unwrap_or("AWSCURRENT");

        if secret.deleted_date.is_some() {
            return Err(SecretsManagerError::SecretMarkedDeleted);
        }

        // Find the version that has the requested stage label
        for version in secret.versions.values() {
            if version.version_stages.contains(&stage.to_string()) {
                return Ok((secret, version));
            }
        }

        Err(SecretsManagerError::SecretVersionNotFoundByStage {
            stage: stage.to_string(),
        })
    }

    pub fn put_secret_value(
        &mut self,
        secret_id: &str,
        secret_string: Option<&str>,
        secret_binary: Option<Vec<u8>>,
    ) -> Result<(&Secret, String), SecretsManagerError> {
        match self.put_secret_value_ext(secret_id, secret_string, secret_binary, None, None) {
            Ok((secret, version_id, _stages)) => Ok((secret, version_id)),
            Err(e) => Err(e),
        }
    }

    pub fn put_secret_value_ext(
        &mut self,
        secret_id: &str,
        secret_string: Option<&str>,
        secret_binary: Option<Vec<u8>>,
        client_request_token: Option<&str>,
        version_stages: Option<&[String]>,
    ) -> Result<(&Secret, String, Vec<String>), SecretsManagerError> {
        let secret = self.find_secret_mut(secret_id)?;

        if secret.deleted_date.is_some() {
            return Err(SecretsManagerError::SecretMarkedDeleted);
        }

        let now = Utc::now();
        let new_version_id = client_request_token
            .map(|s| s.to_string())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let stages = version_stages
            .map(|s| s.to_vec())
            .unwrap_or_else(|| vec!["AWSCURRENT".to_string()]);

        let is_current = stages.contains(&"AWSCURRENT".to_string());

        if is_current {
            // Move AWSCURRENT from old version to AWSPREVIOUS
            if let Some(old_vid) = &secret.current_version_id
                && let Some(old_version) = secret.versions.get_mut(old_vid)
            {
                old_version.version_stages.retain(|s| s != "AWSCURRENT");
                if !old_version
                    .version_stages
                    .contains(&"AWSPREVIOUS".to_string())
                {
                    old_version.version_stages.push("AWSPREVIOUS".to_string());
                }
            }
        }

        // For non-AWSCURRENT stages, remove stage labels from any existing versions that have them
        for stage in &stages {
            if stage != "AWSCURRENT" {
                for existing_version in secret.versions.values_mut() {
                    existing_version.version_stages.retain(|s| s != stage);
                }
            }
        }

        let version = SecretVersion {
            version_id: new_version_id.clone(),
            secret_string: secret_string.map(|s| s.to_string()),
            secret_binary,
            created_date: now,
            version_stages: stages.clone(),
        };

        secret.versions.insert(new_version_id.clone(), version);
        if is_current {
            secret.current_version_id = Some(new_version_id.clone());
        }
        secret.last_changed_date = now;

        // Clean up versions with no stages
        secret.versions.retain(|_k, v| !v.version_stages.is_empty());

        let name = secret.name.clone();
        Ok((self.secrets.get(&name).unwrap(), new_version_id, stages))
    }

    pub fn delete_secret(
        &mut self,
        secret_id: &str,
        recovery_window_in_days: Option<i64>,
        force_delete: bool,
        account_id: &str,
        region: &str,
    ) -> Result<Secret, SecretsManagerError> {
        // Validate recovery window
        if let Some(days) = recovery_window_in_days
            && !(7..=30).contains(&days)
        {
            return Err(SecretsManagerError::InvalidRecoveryWindow);
        }

        // Cannot specify both force delete and recovery window
        if force_delete && recovery_window_in_days.is_some() {
            return Err(SecretsManagerError::ForceDeleteWithRecoveryWindow);
        }

        if force_delete {
            // Force delete - for non-existent secrets, moto returns success with the name
            match self.resolve_name(secret_id) {
                Ok(name) => {
                    let _secret = self.find_secret_mut_raw(&name)?;
                    // Allow force-deleting already-deleted secrets
                    let mut secret = self.secrets.remove(&name).unwrap();
                    secret.deleted_date = Some(Utc::now());
                    return Ok(secret);
                }
                Err(_) => {
                    // For force delete on non-existent secret, moto creates a stub response
                    return Ok(Secret {
                        name: secret_id.to_string(),
                        arn: format!(
                            "arn:aws:secretsmanager:{region}:{account_id}:secret:{secret_id}"
                        ),
                        description: String::new(),
                        created_date: Utc::now(),
                        last_changed_date: Utc::now(),
                        versions: HashMap::new(),
                        current_version_id: None,
                        deleted_date: Some(Utc::now()),
                        tags: HashMap::new(),
                        resource_policy: None,
                        rotation_enabled: None,
                        rotation_lambda_arn: None,
                        rotation_rules: None,
                        last_rotated_date: None,
                        replication_status: Vec::new(),
                        primary_region: None,
                    });
                }
            }
        }

        let secret = self.find_secret_mut(secret_id)?;

        if secret.deleted_date.is_some() {
            return Err(SecretsManagerError::SecretMarkedDeleted);
        }

        let _window = recovery_window_in_days.unwrap_or(30);
        secret.deleted_date = Some(Utc::now());

        Ok(secret.clone())
    }

    fn find_secret_mut_raw(&mut self, name: &str) -> Result<&mut Secret, SecretsManagerError> {
        self.secrets
            .get_mut(name)
            .ok_or(SecretsManagerError::SecretNotFound)
    }

    pub fn restore_secret(&mut self, secret_id: &str) -> Result<&Secret, SecretsManagerError> {
        let secret = self.find_secret_mut(secret_id)?;
        // AWS allows calling RestoreSecret on non-deleted secrets (it's a no-op)
        secret.deleted_date = None;
        let name = secret.name.clone();
        Ok(self.secrets.get(&name).unwrap())
    }

    pub fn describe_secret(&self, secret_id: &str) -> Result<&Secret, SecretsManagerError> {
        self.find_secret(secret_id)
    }

    pub fn list_secrets(&self) -> Vec<&Secret> {
        self.secrets
            .values()
            .filter(|s| s.deleted_date.is_none())
            .collect()
    }

    pub fn update_secret(
        &mut self,
        secret_id: &str,
        description: Option<&str>,
        secret_string: Option<&str>,
        secret_binary: Option<Vec<u8>>,
    ) -> Result<&Secret, SecretsManagerError> {
        let secret = self.find_secret_mut(secret_id)?;

        if let Some(desc) = description {
            secret.description = desc.to_string();
        }

        if secret_string.is_some() || secret_binary.is_some() {
            let now = Utc::now();
            let new_version_id = uuid::Uuid::new_v4().to_string();

            if let Some(old_vid) = &secret.current_version_id
                && let Some(old_version) = secret.versions.get_mut(old_vid)
            {
                old_version.version_stages.retain(|s| s != "AWSCURRENT");
                if !old_version
                    .version_stages
                    .contains(&"AWSPREVIOUS".to_string())
                {
                    old_version.version_stages.push("AWSPREVIOUS".to_string());
                }
            }

            let version = SecretVersion {
                version_id: new_version_id.clone(),
                secret_string: secret_string.map(|s| s.to_string()),
                secret_binary,
                created_date: now,
                version_stages: vec!["AWSCURRENT".to_string()],
            };

            secret.versions.insert(new_version_id.clone(), version);
            secret.current_version_id = Some(new_version_id);
            secret.last_changed_date = now;
        }

        let name = secret.name.clone();
        Ok(self.secrets.get(&name).unwrap())
    }

    pub fn batch_get_secret_value(
        &self,
        secret_ids: &[String],
    ) -> Result<Vec<(&Secret, &SecretVersion)>, Vec<(String, SecretsManagerError)>> {
        let mut values = Vec::new();
        let mut errors = Vec::new();

        for secret_id in secret_ids {
            match self.get_secret_value(secret_id) {
                Ok(pair) => values.push(pair),
                Err(e) => errors.push((secret_id.clone(), e)),
            }
        }

        if values.is_empty() && !errors.is_empty() {
            Err(errors)
        } else {
            Ok(values)
        }
    }

    pub fn cancel_rotate_secret(
        &mut self,
        secret_id: &str,
    ) -> Result<&Secret, SecretsManagerError> {
        let secret = self.find_secret_mut(secret_id)?;
        // Cancel any pending rotation - just return current state
        let name = secret.name.clone();
        Ok(self.secrets.get(&name).unwrap())
    }

    pub fn get_random_password(
        &self,
        password_length: Option<i64>,
        exclude_characters: Option<&str>,
        exclude_numbers: bool,
        exclude_punctuation: bool,
        exclude_uppercase: bool,
        exclude_lowercase: bool,
        include_space: bool,
        require_each_included_type: bool,
    ) -> Result<String, SecretsManagerError> {
        let length = password_length.unwrap_or(32) as usize;
        if length < 4 {
            return Err(SecretsManagerError::PasswordTooShort);
        }
        if length > 4096 {
            return Err(SecretsManagerError::PasswordTooLong);
        }

        let exclude_chars = exclude_characters.unwrap_or("");

        let lowercase = "abcdefghijklmnopqrstuvwxyz";
        let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let digits = "0123456789";
        let punctuation = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

        let mut chars = String::new();
        let mut required_chars: Vec<char> = Vec::new();

        // Space goes first in required chars since it's explicitly requested
        if include_space {
            chars.push(' ');
            required_chars.push(' ');
        }
        if !exclude_lowercase {
            chars.push_str(lowercase);
            let filtered: Vec<char> = lowercase
                .chars()
                .filter(|c| !exclude_chars.contains(*c))
                .collect();
            if !filtered.is_empty() {
                let idx = (uuid::Uuid::new_v4().as_bytes()[0] as usize) % filtered.len();
                required_chars.push(filtered[idx]);
            }
        }
        if !exclude_uppercase {
            chars.push_str(uppercase);
            let filtered: Vec<char> = uppercase
                .chars()
                .filter(|c| !exclude_chars.contains(*c))
                .collect();
            if !filtered.is_empty() {
                let idx = (uuid::Uuid::new_v4().as_bytes()[0] as usize) % filtered.len();
                required_chars.push(filtered[idx]);
            }
        }
        if !exclude_numbers {
            chars.push_str(digits);
            let filtered: Vec<char> = digits
                .chars()
                .filter(|c| !exclude_chars.contains(*c))
                .collect();
            if !filtered.is_empty() {
                let idx = (uuid::Uuid::new_v4().as_bytes()[0] as usize) % filtered.len();
                required_chars.push(filtered[idx]);
            }
        }
        if !exclude_punctuation {
            chars.push_str(punctuation);
            let filtered: Vec<char> = punctuation
                .chars()
                .filter(|c| !exclude_chars.contains(*c))
                .collect();
            if !filtered.is_empty() {
                let idx = (uuid::Uuid::new_v4().as_bytes()[0] as usize) % filtered.len();
                required_chars.push(filtered[idx]);
            }
        }

        if !exclude_chars.is_empty() {
            chars = chars
                .chars()
                .filter(|c| !exclude_chars.contains(*c))
                .collect();
        }

        if chars.is_empty() {
            return Err(SecretsManagerError::NoValidCharsForPassword);
        }

        let char_vec: Vec<char> = chars.chars().collect();
        let mut password: Vec<char> = (0..length)
            .map(|i| {
                let idx = (uuid::Uuid::new_v4().as_bytes()[i % 16] as usize) % char_vec.len();
                char_vec[idx]
            })
            .collect();

        // Ensure each required type is represented
        if require_each_included_type {
            for (i, required) in required_chars.iter().enumerate() {
                if i < password.len() {
                    password[i] = *required;
                }
            }
        }

        Ok(password.into_iter().collect())
    }

    pub fn put_resource_policy(
        &mut self,
        secret_id: &str,
        resource_policy: &str,
    ) -> Result<&Secret, SecretsManagerError> {
        let secret = self.find_secret_mut(secret_id)?;
        secret.resource_policy = Some(resource_policy.to_string());
        secret.last_changed_date = Utc::now();
        let name = secret.name.clone();
        Ok(self.secrets.get(&name).unwrap())
    }

    pub fn get_resource_policy(&self, secret_id: &str) -> Result<&Secret, SecretsManagerError> {
        self.find_secret(secret_id)
    }

    pub fn delete_resource_policy(
        &mut self,
        secret_id: &str,
    ) -> Result<&Secret, SecretsManagerError> {
        let secret = self.find_secret_mut(secret_id)?;
        secret.resource_policy = None;
        secret.last_changed_date = Utc::now();
        let name = secret.name.clone();
        Ok(self.secrets.get(&name).unwrap())
    }

    pub fn list_secret_version_ids(&self, secret_id: &str) -> Result<&Secret, SecretsManagerError> {
        self.find_secret(secret_id)
    }

    pub fn rotate_secret(
        &mut self,
        secret_id: &str,
        rotation_lambda_arn: Option<&str>,
        rotation_rules: Option<RotationRules>,
    ) -> Result<(&Secret, String), SecretsManagerError> {
        let secret = self.find_secret_mut(secret_id)?;

        if secret.deleted_date.is_some() {
            return Err(SecretsManagerError::SecretMarkedDeleted);
        }

        // Validate rotation rules before applying
        if let Some(ref rules) = rotation_rules
            && let Some(days) = rules.automatically_after_days
            && (!(1..=1000).contains(&days))
        {
            return Err(SecretsManagerError::InvalidRotationDays);
        }

        if let Some(arn) = rotation_lambda_arn {
            secret.rotation_lambda_arn = Some(arn.to_string());
        }
        if let Some(rules) = rotation_rules {
            secret.rotation_rules = Some(rules);
        }
        secret.rotation_enabled = Some(true);

        // Create a new version to simulate rotation
        let now = Utc::now();
        let new_version_id = uuid::Uuid::new_v4().to_string();

        // Move AWSCURRENT from old version to AWSPREVIOUS
        if let Some(old_vid) = &secret.current_version_id
            && let Some(old_version) = secret.versions.get_mut(old_vid)
        {
            old_version.version_stages.retain(|s| s != "AWSCURRENT");
            if !old_version
                .version_stages
                .contains(&"AWSPREVIOUS".to_string())
            {
                old_version.version_stages.push("AWSPREVIOUS".to_string());
            }
        }

        // Copy current secret value into new version
        let secret_string = secret
            .current_version_id
            .as_ref()
            .and_then(|vid| secret.versions.get(vid))
            .and_then(|v| v.secret_string.clone());

        let version = SecretVersion {
            version_id: new_version_id.clone(),
            secret_string,
            secret_binary: None,
            created_date: now,
            version_stages: vec!["AWSCURRENT".to_string()],
        };

        secret.versions.insert(new_version_id.clone(), version);
        secret.current_version_id = Some(new_version_id.clone());
        secret.last_changed_date = now;
        secret.last_rotated_date = Some(now);

        let name = secret.name.clone();
        Ok((self.secrets.get(&name).unwrap(), new_version_id))
    }

    pub fn tag_resource(
        &mut self,
        secret_id: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), SecretsManagerError> {
        let secret = self.find_secret_mut(secret_id)?;
        for (k, v) in tags {
            secret.tags.insert(k, v);
        }
        secret.last_changed_date = Utc::now();
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        secret_id: &str,
        tag_keys: &[String],
    ) -> Result<(), SecretsManagerError> {
        let secret = self.find_secret_mut(secret_id)?;
        for key in tag_keys {
            secret.tags.remove(key);
        }
        secret.last_changed_date = Utc::now();
        Ok(())
    }

    pub fn update_secret_version_stage(
        &mut self,
        secret_id: &str,
        version_stage: &str,
        move_to_version_id: Option<&str>,
        remove_from_version_id: Option<&str>,
    ) -> Result<&Secret, SecretsManagerError> {
        let secret = self.find_secret_mut(secret_id)?;

        // If moving AWSCURRENT and no remove_from is specified, require it
        if version_stage == "AWSCURRENT" && remove_from_version_id.is_none() {
            let current_vid = secret
                .versions
                .values()
                .find(|v| v.version_stages.contains(&"AWSCURRENT".to_string()))
                .map(|v| v.version_id.clone());
            if let Some(vid) = current_vid {
                return Err(SecretsManagerError::MissingRemoveFromVersionId { vid });
            }
        }

        if let Some(remove_vid) = remove_from_version_id {
            if let Some(version) = secret.versions.get_mut(remove_vid) {
                version.version_stages.retain(|s| s != version_stage);
            } else {
                return Err(SecretsManagerError::NotAValidVersion {
                    version_id: remove_vid.to_string(),
                });
            }
        }

        if let Some(move_vid) = move_to_version_id {
            if let Some(version) = secret.versions.get_mut(move_vid) {
                if !version.version_stages.contains(&version_stage.to_string()) {
                    version.version_stages.push(version_stage.to_string());
                }
            } else {
                return Err(SecretsManagerError::NotAValidVersion {
                    version_id: move_vid.to_string(),
                });
            }
        }

        // When moving AWSCURRENT, automatically manage AWSPREVIOUS
        if version_stage == "AWSCURRENT" {
            if let Some(remove_vid) = remove_from_version_id {
                let remove_vid = remove_vid.to_string();
                // Remove AWSPREVIOUS from whatever version currently has it
                for version in secret.versions.values_mut() {
                    version.version_stages.retain(|s| s != "AWSPREVIOUS");
                }
                // Add AWSPREVIOUS to the version that AWSCURRENT was removed from
                if let Some(version) = secret.versions.get_mut(&remove_vid) {
                    version.version_stages.push("AWSPREVIOUS".to_string());
                }
            }

            // Remove AWSPREVIOUS from the target version if it has it
            if let Some(move_vid) = move_to_version_id
                && let Some(version) = secret.versions.get_mut(move_vid)
            {
                version.version_stages.retain(|s| s != "AWSPREVIOUS");
            }

            if let Some(move_vid) = move_to_version_id {
                secret.current_version_id = Some(move_vid.to_string());
            }
        }

        secret.last_changed_date = Utc::now();
        let name = secret.name.clone();
        Ok(self.secrets.get(&name).unwrap())
    }

    pub fn replicate_secret_to_regions(
        &mut self,
        secret_id: &str,
        regions: &[String],
    ) -> Result<&Secret, SecretsManagerError> {
        let secret = self.find_secret_mut(secret_id)?;

        for region in regions {
            // Don't add duplicate regions
            if !secret
                .replication_status
                .iter()
                .any(|r| r.region == *region)
            {
                secret.replication_status.push(ReplicationStatus {
                    region: region.clone(),
                    status: "InSync".to_string(),
                    status_message: None,
                    kms_key_id: None,
                    last_accessed_date: None,
                });
            }
        }

        secret.last_changed_date = Utc::now();
        let name = secret.name.clone();
        Ok(self.secrets.get(&name).unwrap())
    }

    pub fn remove_regions_from_replication(
        &mut self,
        secret_id: &str,
        regions: &[String],
    ) -> Result<&Secret, SecretsManagerError> {
        let secret = self.find_secret_mut(secret_id)?;
        secret
            .replication_status
            .retain(|r| !regions.contains(&r.region));
        secret.last_changed_date = Utc::now();
        let name = secret.name.clone();
        Ok(self.secrets.get(&name).unwrap())
    }

    fn find_secret(&self, secret_id: &str) -> Result<&Secret, SecretsManagerError> {
        // Try by name first
        if let Some(secret) = self.secrets.get(secret_id) {
            return Ok(secret);
        }
        // Try by ARN
        for secret in self.secrets.values() {
            if secret.arn == secret_id {
                return Ok(secret);
            }
        }
        Err(SecretsManagerError::SecretNotFound)
    }

    fn find_secret_mut(&mut self, secret_id: &str) -> Result<&mut Secret, SecretsManagerError> {
        let name = self.resolve_name(secret_id)?;
        Ok(self.secrets.get_mut(&name).unwrap())
    }

    fn resolve_name(&self, secret_id: &str) -> Result<String, SecretsManagerError> {
        if self.secrets.contains_key(secret_id) {
            return Ok(secret_id.to_string());
        }
        for secret in self.secrets.values() {
            if secret.arn == secret_id {
                return Ok(secret.name.clone());
            }
        }
        Err(SecretsManagerError::SecretNotFound)
    }
}

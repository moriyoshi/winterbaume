use std::collections::HashMap;

use base64::Engine;
use chrono::Utc;
use sha2::{Digest, Sha256};

use crate::types::*;

/// In-memory state for the ECR service.
#[derive(Debug, Default)]
pub struct EcrState {
    /// Repositories keyed by repository name.
    pub repositories: HashMap<String, Repository>,
    /// Registry-level policy text.
    pub registry_policy: Option<String>,
    /// Registry-level scanning configuration.
    pub registry_scanning_configuration: RegistryScanningConfig,
    /// Registry-level replication configuration.
    pub replication_configuration: ReplicationConfig,
    /// In-progress layer uploads keyed by upload_id.
    pub layer_uploads: HashMap<String, LayerUpload>,
    /// Pull-through cache rules keyed by ecr_repository_prefix.
    pub pull_through_cache_rules: HashMap<String, PullThroughCacheRuleData>,
    /// Repository creation templates keyed by prefix.
    pub repository_creation_templates: HashMap<String, RepositoryCreationTemplateData>,
    /// Signing configuration.
    pub signing_config: SigningConfigData,
    /// Pull-time update exclusions keyed by principal_arn.
    pub pull_time_update_exclusions: HashMap<String, PullTimeUpdateExclusionData>,
    /// Account settings keyed by name.
    pub account_settings: HashMap<String, String>,
}

/// Error type for ECR operations.
#[derive(Debug, thiserror::Error)]
pub enum EcrError {
    #[error(
        "The repository with name '{repository_name}' already exists in the registry with id '{account_id}'"
    )]
    RepositoryAlreadyExists {
        repository_name: String,
        account_id: String,
    },
    #[error(
        "The repository with name '{repository_name}' in registry with id '{registry_id}' cannot be deleted because it still contains images"
    )]
    RepositoryNotEmpty {
        repository_name: String,
        registry_id: String,
    },
    #[error("The repository with name '{repository_name}' does not exist in the registry")]
    RepositoryNotFound { repository_name: String },
    #[error(
        "Image with digest '{digest}' and tag '{tag}' already exists in the repository with name '{repository_name}' in registry with id '{registry_id}'"
    )]
    ImageAlreadyExists {
        digest: String,
        tag: String,
        repository_name: String,
        registry_id: String,
    },
    #[error(
        "The image with imageId {{{image_id_str}}} does not exist within the repository with name '{repository_name}' in the registry with id '{registry_id}'"
    )]
    ImageNotFound {
        image_id_str: String,
        repository_name: String,
        registry_id: String,
    },
    #[error("Repository with name '{repository_name}' does not have a lifecycle policy")]
    LifecyclePolicyNotFound { repository_name: String },
    #[error("Repository policy does not exist for the repository with name '{repository_name}'")]
    RepositoryPolicyNotFound { repository_name: String },
    #[error(
        "Image scan does not exist for the image with '{image_id_str}' in the repository with name '{repository_name}' in the registry with id '{registry_id}'"
    )]
    ScanNotFound {
        image_id_str: String,
        repository_name: String,
        registry_id: String,
    },
    #[error(
        "Invalid parameter at 'resourceArn' failed to satisfy constraint: 'Invalid ARN: {resource_arn}'"
    )]
    InvalidParameter { resource_arn: String },
    #[error("Registry policy does not exist")]
    RegistryPolicyNotFound,
    #[error("Upload not found: {upload_id}")]
    UploadNotFound { upload_id: String },
    #[error("A pull through cache rule for '{prefix}' already exists")]
    PullThroughCacheRuleAlreadyExists { prefix: String },
    #[error("A pull through cache rule for '{prefix}' was not found")]
    PullThroughCacheRuleNotFound { prefix: String },
    #[error("A repository creation template for '{prefix}' already exists")]
    TemplateAlreadyExists { prefix: String },
    #[error("A repository creation template for '{prefix}' was not found")]
    TemplateNotFound { prefix: String },
    #[error("Pull time update exclusion not found: {principal_arn}")]
    PullTimeUpdateExclusionNotFound { principal_arn: String },
}

/// Result of a batch_get_image operation including both successes and failures.
pub struct BatchGetImageResult<'a> {
    pub images: Vec<&'a Image>,
    pub failures: Vec<ImageFailure>,
}

/// Result of a batch_delete_image operation including both successes and failures.
pub struct BatchDeleteImageResult {
    pub image_ids: Vec<ImageId>,
    pub failures: Vec<ImageFailure>,
}

/// An image failure entry for batch operations.
#[derive(Debug, Clone)]
pub struct ImageFailure {
    pub image_id: ImageId,
    pub failure_code: String,
    pub failure_reason: String,
}

/// Result of describe_images.
pub struct DescribeImageDetail<'a> {
    pub image: &'a Image,
    pub repository_name: String,
    pub registry_id: String,
}

/// Result of batch_get_repository_scanning_configuration.
pub struct BatchGetRepoScanConfigResult {
    pub scanning_configurations: Vec<RepoScanConfigEntry>,
    pub failures: Vec<RepoScanConfigFailure>,
}

pub struct RepoScanConfigEntry {
    pub repository_name: String,
    pub repository_arn: String,
    pub scan_on_push: bool,
    pub scan_frequency: String,
    pub applied_scan_filters: Vec<ScanningRepositoryFilterData>,
}

pub struct RepoScanConfigFailure {
    pub repository_name: String,
    pub failure_code: String,
    pub failure_reason: String,
}

impl EcrState {
    pub fn create_repository(
        &mut self,
        repository_name: &str,
        account_id: &str,
        region: &str,
        image_tag_mutability: Option<String>,
        image_scanning_configuration: Option<ImageScanningConfiguration>,
        encryption_configuration: Option<EncryptionConfiguration>,
    ) -> Result<&Repository, EcrError> {
        if self.repositories.contains_key(repository_name) {
            return Err(EcrError::RepositoryAlreadyExists {
                repository_name: repository_name.to_string(),
                account_id: account_id.to_string(),
            });
        }

        let arn = format!("arn:aws:ecr:{region}:{account_id}:repository/{repository_name}");
        let uri = format!("{account_id}.dkr.ecr.{region}.amazonaws.com/{repository_name}");

        let repo = Repository {
            repository_name: repository_name.to_string(),
            repository_arn: arn,
            repository_uri: uri,
            registry_id: account_id.to_string(),
            created_at: Utc::now(),
            images: Vec::new(),
            tags: HashMap::new(),
            lifecycle_policy: None,
            repository_policy: None,
            image_scanning_configuration: image_scanning_configuration.unwrap_or_default(),
            image_tag_mutability: image_tag_mutability.unwrap_or_else(|| "MUTABLE".to_string()),
            encryption_configuration: encryption_configuration.unwrap_or_default(),
        };

        self.repositories.insert(repository_name.to_string(), repo);
        Ok(self.repositories.get(repository_name).unwrap())
    }

    pub fn delete_repository(
        &mut self,
        repository_name: &str,
        force: bool,
    ) -> Result<Repository, EcrError> {
        let repo = self
            .repositories
            .get(repository_name)
            .ok_or_else(|| not_found_error(repository_name))?;

        // Check if repository has images and force is not set
        if !force && !repo.images.is_empty() {
            return Err(EcrError::RepositoryNotEmpty {
                repository_name: repository_name.to_string(),
                registry_id: repo.registry_id.clone(),
            });
        }

        self.repositories
            .remove(repository_name)
            .ok_or_else(|| not_found_error(repository_name))
    }

    pub fn describe_repositories(
        &self,
        names: Option<&[String]>,
    ) -> Result<Vec<&Repository>, EcrError> {
        match names {
            Some(names) => {
                let mut repos = Vec::new();
                for name in names {
                    match self.repositories.get(name.as_str()) {
                        Some(repo) => repos.push(repo),
                        None => return Err(not_found_error(name)),
                    }
                }
                Ok(repos)
            }
            None => Ok(self.repositories.values().collect()),
        }
    }

    /// List images, expanding multi-tagged images into one entry per tag.
    pub fn list_images(&self, repository_name: &str) -> Result<Vec<ListImageEntry>, EcrError> {
        let repo = self
            .repositories
            .get(repository_name)
            .ok_or_else(|| not_found_error(repository_name))?;

        let mut entries = Vec::new();
        for image in &repo.images {
            if image.image_tags.is_empty() {
                // Untagged image - return with digest only
                entries.push(ListImageEntry {
                    image_digest: image.image_digest.clone(),
                    image_tag: None,
                });
            } else {
                // One entry per tag
                for tag in &image.image_tags {
                    entries.push(ListImageEntry {
                        image_digest: image.image_digest.clone(),
                        image_tag: Some(tag.clone()),
                    });
                }
            }
        }
        Ok(entries)
    }

    pub fn put_image(
        &mut self,
        repository_name: &str,
        image_tag: Option<&str>,
        image_manifest: &str,
    ) -> Result<(PutImageResult, String), EcrError> {
        let repo = self
            .repositories
            .get_mut(repository_name)
            .ok_or_else(|| not_found_error(repository_name))?;

        let mut hasher = Sha256::new();
        hasher.update(image_manifest.as_bytes());
        let result = hasher.finalize();
        let digest = format!(
            "sha256:{}",
            result
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect::<String>()
        );

        // Check if image with same digest already exists
        let existing_idx = repo.images.iter().position(|i| i.image_digest == digest);

        if let Some(idx) = existing_idx {
            if let Some(tag) = image_tag {
                // Check if this exact digest+tag combination already exists
                if repo.images[idx].image_tags.contains(&tag.to_string()) {
                    return Err(EcrError::ImageAlreadyExists {
                        digest: digest.clone(),
                        tag: tag.to_string(),
                        repository_name: repository_name.to_string(),
                        registry_id: repo.registry_id.clone(),
                    });
                }

                // Remove this tag from any other image
                for img in repo.images.iter_mut() {
                    img.image_tags.retain(|t| t != tag);
                }
                // Remove images that have no tags left (and were not the target image)
                repo.images
                    .retain(|img| img.image_digest == digest || !img.image_tags.is_empty());

                // Re-find the target image and add the tag
                let existing = repo
                    .images
                    .iter_mut()
                    .find(|i| i.image_digest == digest)
                    .unwrap();
                existing.image_tags.push(tag.to_string());

                let registry_id = repo.registry_id.clone();
                let result = PutImageResult {
                    image_digest: digest,
                    image_tag: Some(tag.to_string()),
                    image_manifest: image_manifest.to_string(),
                };
                return Ok((result, registry_id));
            }
            // No tag specified for existing digest - this is a no-op
            let registry_id = repo.registry_id.clone();
            let result = PutImageResult {
                image_digest: digest,
                image_tag: None,
                image_manifest: image_manifest.to_string(),
            };
            return Ok((result, registry_id));
        }

        // New image
        if let Some(tag) = image_tag {
            // Remove this tag from any other image
            for img in repo.images.iter_mut() {
                img.image_tags.retain(|t| t != tag);
            }
            // Remove images that have no tags left
            repo.images.retain(|img| !img.image_tags.is_empty());
        }

        let tags = match image_tag {
            Some(t) => vec![t.to_string()],
            None => Vec::new(),
        };
        let tag_for_result = image_tag.map(|t| t.to_string());
        repo.images.push(Image {
            image_digest: digest.clone(),
            image_tags: tags,
            image_manifest: image_manifest.to_string(),
            pushed_at: Utc::now(),
            image_scan_status: None,
            image_scan_findings: None,
        });

        let registry_id = repo.registry_id.clone();
        let result = PutImageResult {
            image_digest: digest,
            image_tag: tag_for_result,
            image_manifest: image_manifest.to_string(),
        };
        Ok((result, registry_id))
    }

    pub fn batch_get_image(
        &self,
        repository_name: &str,
        image_ids: &[ImageId],
    ) -> Result<BatchGetImageResult<'_>, EcrError> {
        let repo = self
            .repositories
            .get(repository_name)
            .ok_or_else(|| not_found_error(repository_name))?;

        let mut images = Vec::new();
        let mut failures = Vec::new();

        for id in image_ids {
            let mut found = false;
            for image in &repo.images {
                let matches = match (&id.image_digest, &id.image_tag) {
                    (Some(digest), _) => image.image_digest == *digest,
                    (_, Some(tag)) => image.image_tags.contains(tag),
                    _ => false,
                };
                if matches {
                    images.push(image);
                    found = true;
                    break;
                }
            }
            if !found {
                failures.push(ImageFailure {
                    image_id: id.clone(),
                    failure_code: "ImageNotFound".to_string(),
                    failure_reason: "Requested image not found".to_string(),
                });
            }
        }
        Ok(BatchGetImageResult { images, failures })
    }

    pub fn batch_delete_image(
        &mut self,
        repository_name: &str,
        image_ids: &[ImageId],
    ) -> Result<BatchDeleteImageResult, EcrError> {
        let repo = self
            .repositories
            .get_mut(repository_name)
            .ok_or_else(|| not_found_error(repository_name))?;

        let mut deleted = Vec::new();
        let mut failures = Vec::new();

        for id in image_ids {
            match (&id.image_digest, &id.image_tag) {
                (None, None) => {
                    failures.push(ImageFailure {
                        image_id: id.clone(),
                        failure_code: "MissingDigestAndTag".to_string(),
                        failure_reason:
                            "Invalid request parameters: both tag and digest cannot be null"
                                .to_string(),
                    });
                }
                (Some(digest), tag) => {
                    if let Some(idx) = repo
                        .images
                        .iter()
                        .position(|img| img.image_digest == *digest)
                    {
                        if let Some(tag) = tag
                            && !repo.images[idx].image_tags.contains(tag)
                        {
                            failures.push(ImageFailure {
                                image_id: id.clone(),
                                failure_code: "ImageNotFound".to_string(),
                                failure_reason: "Requested image not found".to_string(),
                            });
                            continue;
                        }
                        let image = repo.images.remove(idx);
                        if image.image_tags.is_empty() {
                            deleted.push(ImageId {
                                image_digest: Some(image.image_digest.clone()),
                                image_tag: None,
                            });
                        } else {
                            for t in &image.image_tags {
                                deleted.push(ImageId {
                                    image_digest: Some(image.image_digest.clone()),
                                    image_tag: Some(t.clone()),
                                });
                            }
                        }
                    } else {
                        failures.push(ImageFailure {
                            image_id: id.clone(),
                            failure_code: "ImageNotFound".to_string(),
                            failure_reason: "Requested image not found".to_string(),
                        });
                    }
                }
                (None, Some(tag)) => {
                    if let Some(image) = repo
                        .images
                        .iter_mut()
                        .find(|img| img.image_tags.contains(tag))
                    {
                        image.image_tags.retain(|t| t != tag);
                        deleted.push(ImageId {
                            image_digest: Some(image.image_digest.clone()),
                            image_tag: Some(tag.clone()),
                        });
                        if image.image_tags.is_empty() {
                            let digest = image.image_digest.clone();
                            repo.images.retain(|img| img.image_digest != digest);
                        }
                    } else {
                        failures.push(ImageFailure {
                            image_id: id.clone(),
                            failure_code: "ImageNotFound".to_string(),
                            failure_reason: "Requested image not found".to_string(),
                        });
                    }
                }
            }
        }
        Ok(BatchDeleteImageResult {
            image_ids: deleted,
            failures,
        })
    }

    pub fn get_authorization_token(
        &self,
        account_id: &str,
        region: &str,
    ) -> AuthorizationTokenData {
        let token = base64::engine::general_purpose::STANDARD
            .encode(format!("AWS:{}", uuid::Uuid::new_v4()));
        AuthorizationTokenData {
            authorization_token: token,
            proxy_endpoint: format!("https://{account_id}.dkr.ecr.{region}.amazonaws.com"),
            expires_at: Utc::now() + chrono::Duration::hours(12),
        }
    }

    // --- New operations ---

    pub fn describe_images(
        &self,
        repository_name: &str,
        image_ids: Option<&[ImageId]>,
    ) -> Result<Vec<DescribeImageDetail<'_>>, EcrError> {
        let repo = self
            .repositories
            .get(repository_name)
            .ok_or_else(|| not_found_error(repository_name))?;

        let images: Vec<&Image> = match image_ids {
            Some(ids) => {
                let mut result = Vec::new();
                for id in ids {
                    let found =
                        repo.images
                            .iter()
                            .find(|img| match (&id.image_digest, &id.image_tag) {
                                (Some(digest), _) => img.image_digest == *digest,
                                (_, Some(tag)) => img.image_tags.contains(tag),
                                _ => false,
                            });
                    match found {
                        Some(img) => result.push(img),
                        None => {
                            return Err(EcrError::ImageNotFound {
                                image_id_str: id
                                    .image_digest
                                    .as_deref()
                                    .or(id.image_tag.as_deref())
                                    .unwrap_or("")
                                    .to_string(),
                                repository_name: repository_name.to_string(),
                                registry_id: repo.registry_id.clone(),
                            });
                        }
                    }
                }
                result
            }
            None => repo.images.iter().collect(),
        };

        Ok(images
            .into_iter()
            .map(|img| DescribeImageDetail {
                image: img,
                repository_name: repository_name.to_string(),
                registry_id: repo.registry_id.clone(),
            })
            .collect())
    }

    pub fn put_lifecycle_policy(
        &mut self,
        repository_name: &str,
        lifecycle_policy_text: &str,
    ) -> Result<(&str, &str), EcrError> {
        let repo = self
            .repositories
            .get_mut(repository_name)
            .ok_or_else(|| not_found_error(repository_name))?;
        repo.lifecycle_policy = Some(lifecycle_policy_text.to_string());
        let registry_id = &repo.registry_id;
        let repo_name = &repo.repository_name;
        Ok((registry_id.as_str(), repo_name.as_str()))
    }

    pub fn get_lifecycle_policy(
        &self,
        repository_name: &str,
    ) -> Result<(&str, &str, &str), EcrError> {
        let repo = self
            .repositories
            .get(repository_name)
            .ok_or_else(|| not_found_error(repository_name))?;
        match &repo.lifecycle_policy {
            Some(policy) => Ok((&repo.registry_id, &repo.repository_name, policy.as_str())),
            None => Err(EcrError::LifecyclePolicyNotFound {
                repository_name: repository_name.to_string(),
            }),
        }
    }

    pub fn delete_lifecycle_policy(
        &mut self,
        repository_name: &str,
    ) -> Result<(String, String, String), EcrError> {
        let repo = self
            .repositories
            .get_mut(repository_name)
            .ok_or_else(|| not_found_error(repository_name))?;
        match repo.lifecycle_policy.take() {
            Some(policy) => Ok((
                repo.registry_id.clone(),
                repo.repository_name.clone(),
                policy,
            )),
            None => Err(EcrError::LifecyclePolicyNotFound {
                repository_name: repository_name.to_string(),
            }),
        }
    }

    pub fn set_repository_policy(
        &mut self,
        repository_name: &str,
        policy_text: &str,
    ) -> Result<(&str, &str), EcrError> {
        let repo = self
            .repositories
            .get_mut(repository_name)
            .ok_or_else(|| not_found_error(repository_name))?;
        repo.repository_policy = Some(policy_text.to_string());
        Ok((&repo.registry_id, &repo.repository_name))
    }

    pub fn get_repository_policy(
        &self,
        repository_name: &str,
    ) -> Result<(&str, &str, &str), EcrError> {
        let repo = self
            .repositories
            .get(repository_name)
            .ok_or_else(|| not_found_error(repository_name))?;
        match &repo.repository_policy {
            Some(policy) => Ok((&repo.registry_id, &repo.repository_name, policy.as_str())),
            None => Err(EcrError::RepositoryPolicyNotFound {
                repository_name: repository_name.to_string(),
            }),
        }
    }

    pub fn delete_repository_policy(
        &mut self,
        repository_name: &str,
    ) -> Result<(String, String, String), EcrError> {
        let repo = self
            .repositories
            .get_mut(repository_name)
            .ok_or_else(|| not_found_error(repository_name))?;
        match repo.repository_policy.take() {
            Some(policy) => Ok((
                repo.registry_id.clone(),
                repo.repository_name.clone(),
                policy,
            )),
            None => Err(EcrError::RepositoryPolicyNotFound {
                repository_name: repository_name.to_string(),
            }),
        }
    }

    pub fn put_image_scanning_configuration(
        &mut self,
        repository_name: &str,
        scan_on_push: bool,
    ) -> Result<(&str, &str, bool), EcrError> {
        let repo = self
            .repositories
            .get_mut(repository_name)
            .ok_or_else(|| not_found_error(repository_name))?;
        repo.image_scanning_configuration.scan_on_push = scan_on_push;
        Ok((
            &repo.registry_id,
            &repo.repository_name,
            repo.image_scanning_configuration.scan_on_push,
        ))
    }

    pub fn put_image_tag_mutability(
        &mut self,
        repository_name: &str,
        image_tag_mutability: &str,
    ) -> Result<(&str, &str, &str), EcrError> {
        let repo = self
            .repositories
            .get_mut(repository_name)
            .ok_or_else(|| not_found_error(repository_name))?;
        repo.image_tag_mutability = image_tag_mutability.to_string();
        Ok((
            &repo.registry_id,
            &repo.repository_name,
            &repo.image_tag_mutability,
        ))
    }

    pub fn start_image_scan(
        &mut self,
        repository_name: &str,
        image_id: &ImageId,
    ) -> Result<(&Image, &str), EcrError> {
        let repo = self
            .repositories
            .get_mut(repository_name)
            .ok_or_else(|| not_found_error(repository_name))?;

        let image = repo.images.iter_mut().find(|img| {
            match (&image_id.image_digest, &image_id.image_tag) {
                (Some(digest), _) => img.image_digest == *digest,
                (_, Some(tag)) => img.image_tags.contains(tag),
                _ => false,
            }
        });

        match image {
            Some(img) => {
                img.image_scan_status = Some(ImageScanStatusData {
                    status: "IN_PROGRESS".to_string(),
                    description: "The scan was initiated.".to_string(),
                });
                img.image_scan_findings = Some(ImageScanFindingsData {
                    image_scan_completed_at: Utc::now(),
                    vulnerability_source_updated_at: Utc::now(),
                    finding_severity_counts: HashMap::new(),
                });
                // Mark as COMPLETE immediately for mock
                img.image_scan_status = Some(ImageScanStatusData {
                    status: "COMPLETE".to_string(),
                    description: "The scan was completed successfully.".to_string(),
                });
                let registry_id = repo.registry_id.as_str();
                Ok((img, registry_id))
            }
            None => Err(EcrError::ImageNotFound {
                image_id_str: image_id
                    .image_digest
                    .as_deref()
                    .or(image_id.image_tag.as_deref())
                    .unwrap_or("")
                    .to_string(),
                repository_name: repository_name.to_string(),
                registry_id: repo.registry_id.clone(),
            }),
        }
    }

    pub fn describe_image_scan_findings(
        &self,
        repository_name: &str,
        image_id: &ImageId,
    ) -> Result<(&Image, &str), EcrError> {
        let repo = self
            .repositories
            .get(repository_name)
            .ok_or_else(|| not_found_error(repository_name))?;

        let image =
            repo.images
                .iter()
                .find(|img| match (&image_id.image_digest, &image_id.image_tag) {
                    (Some(digest), _) => img.image_digest == *digest,
                    (_, Some(tag)) => img.image_tags.contains(tag),
                    _ => false,
                });

        match image {
            Some(img) => {
                if img.image_scan_status.is_none() {
                    return Err(EcrError::ScanNotFound {
                        image_id_str: image_id
                            .image_digest
                            .as_deref()
                            .or(image_id.image_tag.as_deref())
                            .unwrap_or("")
                            .to_string(),
                        repository_name: repository_name.to_string(),
                        registry_id: repo.registry_id.clone(),
                    });
                }
                Ok((img, &repo.registry_id))
            }
            None => Err(EcrError::ImageNotFound {
                image_id_str: image_id
                    .image_digest
                    .as_deref()
                    .or(image_id.image_tag.as_deref())
                    .unwrap_or("")
                    .to_string(),
                repository_name: repository_name.to_string(),
                registry_id: repo.registry_id.clone(),
            }),
        }
    }

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: &[(String, String)],
    ) -> Result<(), EcrError> {
        let repo = self
            .repositories
            .values_mut()
            .find(|r| r.repository_arn == resource_arn);
        match repo {
            Some(r) => {
                for (key, value) in tags {
                    r.tags.insert(key.clone(), value.clone());
                }
                Ok(())
            }
            None => Err(EcrError::InvalidParameter {
                resource_arn: resource_arn.to_string(),
            }),
        }
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), EcrError> {
        let repo = self
            .repositories
            .values_mut()
            .find(|r| r.repository_arn == resource_arn);
        match repo {
            Some(r) => {
                for key in tag_keys {
                    r.tags.remove(key);
                }
                Ok(())
            }
            None => Err(EcrError::InvalidParameter {
                resource_arn: resource_arn.to_string(),
            }),
        }
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<&HashMap<String, String>, EcrError> {
        let repo = self
            .repositories
            .values()
            .find(|r| r.repository_arn == resource_arn);
        match repo {
            Some(r) => Ok(&r.tags),
            None => Err(EcrError::InvalidParameter {
                resource_arn: resource_arn.to_string(),
            }),
        }
    }

    pub fn put_registry_policy(&mut self, policy_text: &str) -> (String, String) {
        self.registry_policy = Some(policy_text.to_string());
        // Return (registry_id, policy_text) - registry_id is inferred
        ("".to_string(), policy_text.to_string())
    }

    pub fn get_registry_policy(&self) -> Result<&str, EcrError> {
        match &self.registry_policy {
            Some(policy) => Ok(policy.as_str()),
            None => Err(EcrError::RegistryPolicyNotFound),
        }
    }

    pub fn delete_registry_policy(&mut self) -> Result<String, EcrError> {
        match self.registry_policy.take() {
            Some(policy) => Ok(policy),
            None => Err(EcrError::RegistryPolicyNotFound),
        }
    }

    pub fn describe_registry(&self) -> &ReplicationConfig {
        &self.replication_configuration
    }

    pub fn put_registry_scanning_configuration(
        &mut self,
        scan_type: Option<&str>,
        rules: Vec<RegistryScanningRuleData>,
    ) -> &RegistryScanningConfig {
        if let Some(st) = scan_type {
            self.registry_scanning_configuration.scan_type = st.to_string();
        }
        self.registry_scanning_configuration.rules = rules;
        &self.registry_scanning_configuration
    }

    pub fn get_registry_scanning_configuration(&self) -> &RegistryScanningConfig {
        &self.registry_scanning_configuration
    }

    pub fn put_replication_configuration(
        &mut self,
        rules: Vec<ReplicationRuleData>,
    ) -> &ReplicationConfig {
        self.replication_configuration.rules = rules;
        &self.replication_configuration
    }

    // --- Layer upload operations ---

    pub fn initiate_layer_upload(
        &mut self,
        registry_id: &str,
        repository_name: &str,
    ) -> Result<String, EcrError> {
        if !self.repositories.contains_key(repository_name) {
            return Err(not_found_error(repository_name));
        }
        let upload_id = uuid::Uuid::new_v4().to_string();
        self.layer_uploads.insert(
            upload_id.clone(),
            LayerUpload {
                upload_id: upload_id.clone(),
                registry_id: registry_id.to_string(),
                repository_name: repository_name.to_string(),
                last_byte_received: 0,
            },
        );
        Ok(upload_id)
    }

    pub fn upload_layer_part(
        &mut self,
        upload_id: &str,
        _layer_part_blob: &[u8],
        _part_first_byte: i64,
        part_last_byte: i64,
    ) -> Result<i64, EcrError> {
        let upload =
            self.layer_uploads
                .get_mut(upload_id)
                .ok_or_else(|| EcrError::UploadNotFound {
                    upload_id: upload_id.to_string(),
                })?;
        upload.last_byte_received = part_last_byte;
        Ok(part_last_byte)
    }

    pub fn complete_layer_upload(
        &mut self,
        upload_id: &str,
        layer_digests: &[String],
        account_id: &str,
    ) -> Result<(String, String, String), EcrError> {
        let upload =
            self.layer_uploads
                .remove(upload_id)
                .ok_or_else(|| EcrError::UploadNotFound {
                    upload_id: upload_id.to_string(),
                })?;
        let layer_digest = layer_digests
            .first()
            .cloned()
            .unwrap_or_else(|| format!("sha256:{}", uuid::Uuid::new_v4().simple()));
        Ok((
            account_id.to_string(),
            upload.repository_name.clone(),
            layer_digest,
        ))
    }

    pub fn batch_check_layer_availability(
        &self,
        repository_name: &str,
        layer_digests: &[String],
    ) -> Result<(Vec<(String, String)>, Vec<String>), EcrError> {
        if !self.repositories.contains_key(repository_name) {
            return Err(not_found_error(repository_name));
        }
        // For simplicity, report all layers as UNAVAILABLE (mock doesn't store actual layers)
        let layers: Vec<(String, String)> = layer_digests
            .iter()
            .map(|d| (d.clone(), "UNAVAILABLE".to_string()))
            .collect();
        Ok((layers, Vec::new()))
    }

    pub fn get_download_url_for_layer(
        &self,
        repository_name: &str,
        layer_digest: &str,
        account_id: &str,
        region: &str,
    ) -> Result<String, EcrError> {
        if !self.repositories.contains_key(repository_name) {
            return Err(not_found_error(repository_name));
        }
        let url = format!(
            "https://{account_id}.dkr.ecr.{region}.amazonaws.com/v2/{repository_name}/blobs/{layer_digest}"
        );
        Ok(url)
    }

    // --- Pull-through cache rules ---

    pub fn create_pull_through_cache_rule(
        &mut self,
        ecr_repository_prefix: &str,
        upstream_registry_url: &str,
        registry_id: &str,
        upstream_registry: Option<&str>,
        upstream_repository_prefix: Option<&str>,
        credential_arn: Option<&str>,
        custom_role_arn: Option<&str>,
    ) -> Result<&PullThroughCacheRuleData, EcrError> {
        if self
            .pull_through_cache_rules
            .contains_key(ecr_repository_prefix)
        {
            return Err(EcrError::PullThroughCacheRuleAlreadyExists {
                prefix: ecr_repository_prefix.to_string(),
            });
        }
        let rule = PullThroughCacheRuleData {
            ecr_repository_prefix: ecr_repository_prefix.to_string(),
            upstream_registry_url: upstream_registry_url.to_string(),
            created_at: Utc::now(),
            registry_id: registry_id.to_string(),
            upstream_registry: upstream_registry.map(|s| s.to_string()),
            upstream_repository_prefix: upstream_repository_prefix.map(|s| s.to_string()),
            credential_arn: credential_arn.map(|s| s.to_string()),
            custom_role_arn: custom_role_arn.map(|s| s.to_string()),
        };
        self.pull_through_cache_rules
            .insert(ecr_repository_prefix.to_string(), rule);
        Ok(self
            .pull_through_cache_rules
            .get(ecr_repository_prefix)
            .unwrap())
    }

    pub fn delete_pull_through_cache_rule(
        &mut self,
        ecr_repository_prefix: &str,
    ) -> Result<PullThroughCacheRuleData, EcrError> {
        self.pull_through_cache_rules
            .remove(ecr_repository_prefix)
            .ok_or_else(|| EcrError::PullThroughCacheRuleNotFound {
                prefix: ecr_repository_prefix.to_string(),
            })
    }

    pub fn describe_pull_through_cache_rules(
        &self,
        prefixes: Option<&[String]>,
    ) -> Vec<&PullThroughCacheRuleData> {
        match prefixes {
            Some(ps) => ps
                .iter()
                .filter_map(|p| self.pull_through_cache_rules.get(p.as_str()))
                .collect(),
            None => self.pull_through_cache_rules.values().collect(),
        }
    }

    pub fn update_pull_through_cache_rule(
        &mut self,
        ecr_repository_prefix: &str,
        credential_arn: Option<&str>,
        custom_role_arn: Option<&str>,
        registry_id: &str,
    ) -> Result<&PullThroughCacheRuleData, EcrError> {
        let rule = self
            .pull_through_cache_rules
            .get_mut(ecr_repository_prefix)
            .ok_or_else(|| EcrError::PullThroughCacheRuleNotFound {
                prefix: ecr_repository_prefix.to_string(),
            })?;
        if let Some(a) = credential_arn {
            rule.credential_arn = Some(a.to_string());
        }
        if let Some(a) = custom_role_arn {
            rule.custom_role_arn = Some(a.to_string());
        }
        let _ = registry_id;
        Ok(self
            .pull_through_cache_rules
            .get(ecr_repository_prefix)
            .unwrap())
    }

    pub fn validate_pull_through_cache_rule(
        &self,
        ecr_repository_prefix: &str,
        registry_id: &str,
    ) -> Result<&PullThroughCacheRuleData, EcrError> {
        let _ = registry_id;
        self.pull_through_cache_rules
            .get(ecr_repository_prefix)
            .ok_or_else(|| EcrError::PullThroughCacheRuleNotFound {
                prefix: ecr_repository_prefix.to_string(),
            })
    }

    // --- Repository creation templates ---

    pub fn create_repository_creation_template(
        &mut self,
        prefix: &str,
        description: Option<&str>,
        lifecycle_policy: Option<&str>,
        repository_policy: Option<&str>,
        image_tag_mutability: Option<&str>,
        custom_role_arn: Option<&str>,
        applied_for: Vec<String>,
    ) -> Result<&RepositoryCreationTemplateData, EcrError> {
        if self.repository_creation_templates.contains_key(prefix) {
            return Err(EcrError::TemplateAlreadyExists {
                prefix: prefix.to_string(),
            });
        }
        let now = Utc::now();
        let template = RepositoryCreationTemplateData {
            prefix: prefix.to_string(),
            description: description.map(|s| s.to_string()),
            lifecycle_policy: lifecycle_policy.map(|s| s.to_string()),
            repository_policy: repository_policy.map(|s| s.to_string()),
            image_tag_mutability: image_tag_mutability.map(|s| s.to_string()),
            custom_role_arn: custom_role_arn.map(|s| s.to_string()),
            applied_for,
            created_at: now,
            updated_at: now,
        };
        self.repository_creation_templates
            .insert(prefix.to_string(), template);
        Ok(self.repository_creation_templates.get(prefix).unwrap())
    }

    pub fn delete_repository_creation_template(
        &mut self,
        prefix: &str,
    ) -> Result<RepositoryCreationTemplateData, EcrError> {
        self.repository_creation_templates
            .remove(prefix)
            .ok_or_else(|| EcrError::TemplateNotFound {
                prefix: prefix.to_string(),
            })
    }

    pub fn describe_repository_creation_templates(
        &self,
        prefixes: Option<&[String]>,
    ) -> Vec<&RepositoryCreationTemplateData> {
        match prefixes {
            Some(ps) => ps
                .iter()
                .filter_map(|p| self.repository_creation_templates.get(p.as_str()))
                .collect(),
            None => self.repository_creation_templates.values().collect(),
        }
    }

    pub fn update_repository_creation_template(
        &mut self,
        prefix: &str,
        description: Option<&str>,
        lifecycle_policy: Option<&str>,
        repository_policy: Option<&str>,
        image_tag_mutability: Option<&str>,
        custom_role_arn: Option<&str>,
        applied_for: Option<Vec<String>>,
    ) -> Result<&RepositoryCreationTemplateData, EcrError> {
        let t = self
            .repository_creation_templates
            .get_mut(prefix)
            .ok_or_else(|| EcrError::TemplateNotFound {
                prefix: prefix.to_string(),
            })?;
        if let Some(v) = description {
            t.description = Some(v.to_string());
        }
        if let Some(v) = lifecycle_policy {
            t.lifecycle_policy = Some(v.to_string());
        }
        if let Some(v) = repository_policy {
            t.repository_policy = Some(v.to_string());
        }
        if let Some(v) = image_tag_mutability {
            t.image_tag_mutability = Some(v.to_string());
        }
        if let Some(v) = custom_role_arn {
            t.custom_role_arn = Some(v.to_string());
        }
        if let Some(v) = applied_for {
            t.applied_for = v;
        }
        t.updated_at = Utc::now();
        Ok(self.repository_creation_templates.get(prefix).unwrap())
    }

    // --- Signing configuration ---

    pub fn get_signing_configuration(&self) -> &SigningConfigData {
        &self.signing_config
    }

    pub fn put_signing_configuration(&mut self, rules: Vec<SigningRuleData>) -> &SigningConfigData {
        self.signing_config.rules = rules;
        &self.signing_config
    }

    pub fn delete_signing_configuration(&mut self) {
        self.signing_config.rules.clear();
    }

    // --- Pull-time update exclusions ---

    pub fn register_pull_time_update_exclusion(
        &mut self,
        principal_arn: &str,
    ) -> &PullTimeUpdateExclusionData {
        let entry = PullTimeUpdateExclusionData {
            principal_arn: principal_arn.to_string(),
            created_at: Utc::now(),
        };
        self.pull_time_update_exclusions
            .insert(principal_arn.to_string(), entry);
        self.pull_time_update_exclusions.get(principal_arn).unwrap()
    }

    pub fn deregister_pull_time_update_exclusion(
        &mut self,
        principal_arn: &str,
    ) -> Result<String, EcrError> {
        self.pull_time_update_exclusions
            .remove(principal_arn)
            .map(|e| e.principal_arn)
            .ok_or_else(|| EcrError::PullTimeUpdateExclusionNotFound {
                principal_arn: principal_arn.to_string(),
            })
    }

    pub fn list_pull_time_update_exclusions(&self) -> Vec<String> {
        self.pull_time_update_exclusions.keys().cloned().collect()
    }

    // --- Account settings ---

    pub fn get_account_setting(&self, name: &str) -> Option<&String> {
        self.account_settings.get(name)
    }

    pub fn put_account_setting(&mut self, name: &str, value: &str) -> (&str, &str) {
        self.account_settings
            .insert(name.to_string(), value.to_string());
        let entry = self.account_settings.get_key_value(name).unwrap();
        (entry.0.as_str(), entry.1.as_str())
    }

    pub fn batch_get_repository_scanning_configuration(
        &self,
        repository_names: &[String],
    ) -> BatchGetRepoScanConfigResult {
        let mut configs = Vec::new();
        let mut failures = Vec::new();

        for name in repository_names {
            match self.repositories.get(name.as_str()) {
                Some(repo) => {
                    // Find matching scanning rules for this repo
                    let mut applied_filters = Vec::new();
                    let mut scan_frequency = "MANUAL".to_string();
                    for rule in &self.registry_scanning_configuration.rules {
                        for filter in &rule.repository_filters {
                            if name.starts_with(&filter.filter) || filter.filter == "*" {
                                applied_filters.push(filter.clone());
                                scan_frequency = rule.scan_frequency.clone();
                            }
                        }
                    }
                    configs.push(RepoScanConfigEntry {
                        repository_name: repo.repository_name.clone(),
                        repository_arn: repo.repository_arn.clone(),
                        scan_on_push: repo.image_scanning_configuration.scan_on_push,
                        scan_frequency,
                        applied_scan_filters: applied_filters,
                    });
                }
                None => {
                    failures.push(RepoScanConfigFailure {
                        repository_name: name.clone(),
                        failure_code: "REPOSITORY_NOT_FOUND".to_string(),
                        failure_reason: format!(
                            "REPOSITORY_NOT_FOUND: repository with name '{name}' not found"
                        ),
                    });
                }
            }
        }

        BatchGetRepoScanConfigResult {
            scanning_configurations: configs,
            failures,
        }
    }
}

/// Image identifier for batch operations.
#[derive(Debug, Clone)]
pub struct ImageId {
    pub image_digest: Option<String>,
    pub image_tag: Option<String>,
}

/// Result of a put_image operation (avoids lifetime issues with returning references).
pub struct PutImageResult {
    pub image_digest: String,
    pub image_tag: Option<String>,
    pub image_manifest: String,
}

/// An entry in a list_images response (one per tag).
pub struct ListImageEntry {
    pub image_digest: String,
    pub image_tag: Option<String>,
}

/// Authorization data returned by GetAuthorizationToken.
pub struct AuthorizationTokenData {
    pub authorization_token: String,
    pub proxy_endpoint: String,
    pub expires_at: chrono::DateTime<Utc>,
}

fn not_found_error(repository_name: &str) -> EcrError {
    EcrError::RepositoryNotFound {
        repository_name: repository_name.to_string(),
    }
}

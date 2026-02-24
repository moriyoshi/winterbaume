use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct LakeFormationState {
    /// Registered resources keyed by resource ARN.
    pub resources: HashMap<String, RegisteredResource>,
    /// Data lake settings for this account/region.
    pub data_lake_settings: DataLakeSettings,
    /// LF-Tags keyed by tag key.
    pub lf_tags: HashMap<String, LFTag>,
    /// LF-Tag assignments keyed by a resource identifier string.
    pub tag_assignments: HashMap<String, Vec<LFTagPair>>,
    /// Permission grants.
    pub permissions: Vec<PermissionGrant>,
}

#[derive(Debug, Error)]
pub enum LakeFormationError {
    #[error("Resource {resource_arn} is already registered.")]
    ResourceAlreadyExists { resource_arn: String },

    #[error("Resource {resource_arn} is not registered.")]
    ResourceNotFound { resource_arn: String },

    #[error("Tag {tag_key} already exists.")]
    TagAlreadyExists { tag_key: String },

    #[error("Tag {tag_key} not found.")]
    TagNotFound { tag_key: String },
}

impl LakeFormationState {
    pub fn register_resource(
        &mut self,
        resource_arn: &str,
        role_arn: Option<&str>,
        use_service_linked_role: bool,
    ) -> Result<(), LakeFormationError> {
        if self.resources.contains_key(resource_arn) {
            return Err(LakeFormationError::ResourceAlreadyExists {
                resource_arn: resource_arn.to_string(),
            });
        }

        let resource = RegisteredResource {
            resource_arn: resource_arn.to_string(),
            role_arn: role_arn.map(String::from),
            use_service_linked_role,
        };

        self.resources.insert(resource_arn.to_string(), resource);
        Ok(())
    }

    pub fn deregister_resource(&mut self, resource_arn: &str) -> Result<(), LakeFormationError> {
        if self.resources.remove(resource_arn).is_none() {
            return Err(LakeFormationError::ResourceNotFound {
                resource_arn: resource_arn.to_string(),
            });
        }
        Ok(())
    }

    pub fn describe_resource(
        &self,
        resource_arn: &str,
    ) -> Result<&RegisteredResource, LakeFormationError> {
        self.resources
            .get(resource_arn)
            .ok_or(LakeFormationError::ResourceNotFound {
                resource_arn: resource_arn.to_string(),
            })
    }

    pub fn list_resources(&self) -> Vec<&RegisteredResource> {
        self.resources.values().collect()
    }

    pub fn get_data_lake_settings(&self) -> &DataLakeSettings {
        &self.data_lake_settings
    }

    pub fn put_data_lake_settings(&mut self, settings: DataLakeSettings) {
        self.data_lake_settings = settings;
    }

    pub fn create_lf_tag(
        &mut self,
        tag_key: &str,
        tag_values: Vec<String>,
        catalog_id: Option<String>,
    ) -> Result<(), LakeFormationError> {
        if self.lf_tags.contains_key(tag_key) {
            return Err(LakeFormationError::TagAlreadyExists {
                tag_key: tag_key.to_string(),
            });
        }
        self.lf_tags.insert(
            tag_key.to_string(),
            LFTag {
                tag_key: tag_key.to_string(),
                tag_values,
                catalog_id,
            },
        );
        Ok(())
    }

    pub fn delete_lf_tag(&mut self, tag_key: &str) -> Result<(), LakeFormationError> {
        if self.lf_tags.remove(tag_key).is_none() {
            return Err(LakeFormationError::TagNotFound {
                tag_key: tag_key.to_string(),
            });
        }
        Ok(())
    }

    pub fn get_lf_tag(&self, tag_key: &str) -> Result<&LFTag, LakeFormationError> {
        self.lf_tags
            .get(tag_key)
            .ok_or(LakeFormationError::TagNotFound {
                tag_key: tag_key.to_string(),
            })
    }

    pub fn list_lf_tags(&self) -> Vec<&LFTag> {
        self.lf_tags.values().collect()
    }

    pub fn add_lf_tags_to_resource(&mut self, resource_key: &str, tags: Vec<LFTagPair>) {
        let entry = self
            .tag_assignments
            .entry(resource_key.to_string())
            .or_default();
        for tag in tags {
            // Replace existing tag with same key, or add new
            if let Some(existing) = entry.iter_mut().find(|t| t.tag_key == tag.tag_key) {
                existing.tag_values = tag.tag_values;
            } else {
                entry.push(tag);
            }
        }
    }

    pub fn get_resource_lf_tags(&self, resource_key: &str) -> Vec<&LFTagPair> {
        self.tag_assignments
            .get(resource_key)
            .map(|tags| tags.iter().collect())
            .unwrap_or_default()
    }

    pub fn update_lf_tag(
        &mut self,
        tag_key: &str,
        tags_to_add: Vec<String>,
        tags_to_delete: Vec<String>,
    ) -> Result<(), LakeFormationError> {
        let tag = self
            .lf_tags
            .get_mut(tag_key)
            .ok_or(LakeFormationError::TagNotFound {
                tag_key: tag_key.to_string(),
            })?;
        tag.tag_values.retain(|v| !tags_to_delete.contains(v));
        for v in tags_to_add {
            if !tag.tag_values.contains(&v) {
                tag.tag_values.push(v);
            }
        }
        Ok(())
    }

    pub fn remove_lf_tags_from_resource(&mut self, resource_key: &str, tag_keys: Vec<String>) {
        if let Some(entry) = self.tag_assignments.get_mut(resource_key) {
            entry.retain(|t| !tag_keys.contains(&t.tag_key));
        }
    }

    pub fn grant_permissions(
        &mut self,
        principal: &str,
        resource: serde_json::Value,
        permissions: Vec<String>,
        permissions_with_grant_option: Vec<String>,
    ) {
        // Find existing grant for same principal+resource and merge
        if let Some(existing) = self
            .permissions
            .iter_mut()
            .find(|p| p.principal == principal && p.resource == resource)
        {
            for perm in permissions {
                if !existing.permissions.contains(&perm) {
                    existing.permissions.push(perm);
                }
            }
            for perm in permissions_with_grant_option {
                if !existing.permissions_with_grant_option.contains(&perm) {
                    existing.permissions_with_grant_option.push(perm);
                }
            }
        } else {
            self.permissions.push(PermissionGrant {
                principal: principal.to_string(),
                resource,
                permissions,
                permissions_with_grant_option,
            });
        }
    }

    pub fn revoke_permissions(
        &mut self,
        principal: &str,
        resource: &serde_json::Value,
        permissions_to_revoke: &[String],
    ) {
        // Remove specific permissions from the matching grant entry
        // Keep entry if it still has some permissions remaining
        self.permissions.retain_mut(|p| {
            if p.principal == principal && p.resource == *resource {
                p.permissions
                    .retain(|perm| !permissions_to_revoke.contains(perm));
                // Keep the entry if it still has permissions
                !p.permissions.is_empty()
            } else {
                true
            }
        });
    }

    pub fn list_permissions(&self, principal: Option<&str>) -> Vec<&PermissionGrant> {
        self.permissions
            .iter()
            .filter(|p| principal.is_none_or(|princ| p.principal == princ))
            .collect()
    }
}

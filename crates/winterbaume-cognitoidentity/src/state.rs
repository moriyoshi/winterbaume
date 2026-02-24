use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct CognitoIdentityState {
    pub identity_pools: HashMap<String, IdentityPool>,
    /// Map from pool_id to list of identities
    pub identities: HashMap<String, Vec<Identity>>,
    /// Map from identity_id to pool_id (reverse index)
    pub identity_index: HashMap<String, String>,
    /// Map from pool_id to roles
    pub pool_roles: HashMap<String, IdentityPoolRoles>,
    /// Map from (pool_id + "\x00" + provider_name) to principal tag entry
    pub principal_tags: HashMap<String, PrincipalTagEntry>,
    /// Map from resource ARN to tags
    pub resource_tags: HashMap<String, HashMap<String, String>>,
    /// Developer identity links: keyed by "{pool_id}\x00{developer_user_identifier}"
    pub developer_identities: HashMap<String, crate::types::DeveloperIdentityLink>,
}

#[derive(Debug, Error)]
pub enum CognitoIdentityError {
    #[error(
        "1 validation error detected: Value '{name}' at 'identityPoolName' failed to satisfy constraint: Member must satisfy regular expression pattern: [\\w\\s+=,.@-]+"
    )]
    InvalidPoolName { name: String },

    #[error("{resource_id}")]
    ResourceNotFound { resource_id: String },
}

impl CognitoIdentityState {
    pub fn create_identity_pool(
        &mut self,
        name: &str,
        allow_unauthenticated: bool,
        supported_login_providers: HashMap<String, String>,
        developer_provider_name: Option<String>,
        open_id_connect_provider_arns: Vec<String>,
        cognito_identity_providers: Vec<CognitoIdentityProvider>,
        saml_provider_arns: Vec<String>,
        region: &str,
    ) -> Result<&IdentityPool, CognitoIdentityError> {
        // Validate name
        let re = regex::Regex::new(r"^[\w\s+=,.@-]+$").unwrap();
        if !re.is_match(name) {
            return Err(CognitoIdentityError::InvalidPoolName {
                name: name.to_string(),
            });
        }

        let pool_id = format!("{}:{}", region, uuid::Uuid::new_v4());
        let pool = IdentityPool {
            identity_pool_id: pool_id.clone(),
            identity_pool_name: name.to_string(),
            allow_unauthenticated_identities: allow_unauthenticated,
            supported_login_providers,
            developer_provider_name,
            open_id_connect_provider_arns,
            cognito_identity_providers,
            saml_provider_arns,
            created_date: Utc::now(),
        };

        self.identity_pools.insert(pool_id.clone(), pool);
        Ok(self.identity_pools.get(&pool_id).unwrap())
    }

    pub fn describe_identity_pool(
        &self,
        pool_id: &str,
    ) -> Result<&IdentityPool, CognitoIdentityError> {
        self.identity_pools
            .get(pool_id)
            .ok_or_else(|| CognitoIdentityError::ResourceNotFound {
                resource_id: pool_id.to_string(),
            })
    }

    pub fn update_identity_pool(
        &mut self,
        pool_id: &str,
        name: &str,
        allow_unauthenticated: bool,
        supported_login_providers: Option<HashMap<String, String>>,
        developer_provider_name: Option<String>,
        open_id_connect_provider_arns: Option<Vec<String>>,
        cognito_identity_providers: Option<Vec<CognitoIdentityProvider>>,
        saml_provider_arns: Option<Vec<String>>,
    ) -> Result<&IdentityPool, CognitoIdentityError> {
        let pool = self.identity_pools.get_mut(pool_id).ok_or_else(|| {
            CognitoIdentityError::ResourceNotFound {
                resource_id: pool_id.to_string(),
            }
        })?;

        pool.identity_pool_name = name.to_string();
        pool.allow_unauthenticated_identities = allow_unauthenticated;
        if let Some(providers) = supported_login_providers {
            pool.supported_login_providers = providers;
        }
        if let Some(dev_name) = developer_provider_name {
            pool.developer_provider_name = Some(dev_name);
        }
        if let Some(arns) = open_id_connect_provider_arns {
            pool.open_id_connect_provider_arns = arns;
        }
        if let Some(providers) = cognito_identity_providers {
            pool.cognito_identity_providers = providers;
        }
        if let Some(arns) = saml_provider_arns {
            pool.saml_provider_arns = arns;
        }

        Ok(self.identity_pools.get(pool_id).unwrap())
    }

    pub fn list_identity_pools(&self) -> Vec<&IdentityPool> {
        self.identity_pools.values().collect()
    }

    pub fn delete_identity_pool(&mut self, pool_id: &str) -> Result<(), CognitoIdentityError> {
        self.identity_pools.remove(pool_id).ok_or_else(|| {
            CognitoIdentityError::ResourceNotFound {
                resource_id: pool_id.to_string(),
            }
        })?;
        self.identities.remove(pool_id);
        Ok(())
    }

    pub fn get_id(&mut self, pool_id: &str, region: &str) -> Result<String, CognitoIdentityError> {
        // Check that the pool exists
        if !self.identity_pools.contains_key(pool_id) {
            return Err(CognitoIdentityError::ResourceNotFound {
                resource_id: pool_id.to_string(),
            });
        }

        let now = Utc::now();
        let identity_id = format!("{}:{}", region, uuid::Uuid::new_v4());
        let identity = Identity {
            identity_id: identity_id.clone(),
            identity_pool_id: pool_id.to_string(),
            logins: Vec::new(),
            creation_date: now,
            last_modified_date: now,
        };
        self.identity_index
            .insert(identity_id.clone(), pool_id.to_string());
        self.identities
            .entry(pool_id.to_string())
            .or_default()
            .push(identity);
        Ok(identity_id)
    }

    pub fn delete_identities(&mut self, identity_ids: &[String]) -> Vec<String> {
        // Returns list of identity_ids that were NOT found (unprocessed)
        let mut unprocessed = Vec::new();
        for id in identity_ids {
            if let Some(pool_id) = self.identity_index.remove(id) {
                if let Some(identities) = self.identities.get_mut(&pool_id) {
                    identities.retain(|i| &i.identity_id != id);
                }
            } else {
                unprocessed.push(id.clone());
            }
        }
        unprocessed
    }

    pub fn describe_identity(&self, identity_id: &str) -> Result<&Identity, CognitoIdentityError> {
        let pool_id = self.identity_index.get(identity_id).ok_or_else(|| {
            CognitoIdentityError::ResourceNotFound {
                resource_id: identity_id.to_string(),
            }
        })?;
        self.identities
            .get(pool_id)
            .and_then(|v| v.iter().find(|i| i.identity_id == identity_id))
            .ok_or_else(|| CognitoIdentityError::ResourceNotFound {
                resource_id: identity_id.to_string(),
            })
    }

    pub fn get_identity_pool_roles(
        &self,
        pool_id: &str,
    ) -> Result<IdentityPoolRoles, CognitoIdentityError> {
        if !self.identity_pools.contains_key(pool_id) {
            return Err(CognitoIdentityError::ResourceNotFound {
                resource_id: pool_id.to_string(),
            });
        }
        Ok(self.pool_roles.get(pool_id).cloned().unwrap_or_default())
    }

    pub fn set_identity_pool_roles(
        &mut self,
        pool_id: &str,
        roles: HashMap<String, String>,
    ) -> Result<(), CognitoIdentityError> {
        if !self.identity_pools.contains_key(pool_id) {
            return Err(CognitoIdentityError::ResourceNotFound {
                resource_id: pool_id.to_string(),
            });
        }
        let entry = self.pool_roles.entry(pool_id.to_string()).or_default();
        entry.roles = roles;
        Ok(())
    }

    pub fn get_principal_tag_attribute_map(
        &self,
        pool_id: &str,
        provider_name: &str,
    ) -> Option<&PrincipalTagEntry> {
        let key = format!("{}\x00{}", pool_id, provider_name);
        self.principal_tags.get(&key)
    }

    pub fn set_principal_tag_attribute_map(
        &mut self,
        pool_id: &str,
        provider_name: &str,
        use_defaults: bool,
        principal_tags: HashMap<String, String>,
    ) {
        let key = format!("{}\x00{}", pool_id, provider_name);
        self.principal_tags.insert(
            key,
            PrincipalTagEntry {
                identity_pool_id: pool_id.to_string(),
                identity_provider_name: provider_name.to_string(),
                use_defaults,
                principal_tags,
            },
        );
    }

    pub fn tag_resource(&mut self, resource_arn: &str, tags: HashMap<String, String>) {
        let entry = self
            .resource_tags
            .entry(resource_arn.to_string())
            .or_default();
        entry.extend(tags);
    }

    pub fn untag_resource(&mut self, resource_arn: &str, tag_keys: &[String]) {
        if let Some(entry) = self.resource_tags.get_mut(resource_arn) {
            for key in tag_keys {
                entry.remove(key);
            }
        }
    }

    pub fn list_tags_for_resource(&self, resource_arn: &str) -> HashMap<String, String> {
        self.resource_tags
            .get(resource_arn)
            .cloned()
            .unwrap_or_default()
    }

    pub fn list_identities(&self, pool_id: &str) -> Result<Vec<&Identity>, CognitoIdentityError> {
        if !self.identity_pools.contains_key(pool_id) {
            return Err(CognitoIdentityError::ResourceNotFound {
                resource_id: pool_id.to_string(),
            });
        }
        Ok(self
            .identities
            .get(pool_id)
            .map(|v| v.iter().collect())
            .unwrap_or_default())
    }

    // --- Developer Identity operations ---

    fn dev_identity_key(pool_id: &str, developer_user_identifier: &str) -> String {
        format!("{pool_id}\x00{developer_user_identifier}")
    }

    pub fn lookup_developer_identity(
        &self,
        pool_id: &str,
        identity_id: Option<&str>,
        developer_user_identifier: Option<&str>,
    ) -> Result<(String, Vec<String>), CognitoIdentityError> {
        if !self.identity_pools.contains_key(pool_id) {
            return Err(CognitoIdentityError::ResourceNotFound {
                resource_id: pool_id.to_string(),
            });
        }

        if let Some(dev_uid) = developer_user_identifier {
            let key = Self::dev_identity_key(pool_id, dev_uid);
            if let Some(link) = self.developer_identities.get(&key) {
                return Ok((link.identity_id.clone(), vec![dev_uid.to_string()]));
            }
            return Err(CognitoIdentityError::ResourceNotFound {
                resource_id: dev_uid.to_string(),
            });
        }

        if let Some(iid) = identity_id {
            let dev_uids: Vec<String> = self
                .developer_identities
                .values()
                .filter(|link| link.identity_pool_id == pool_id && link.identity_id == iid)
                .map(|link| link.developer_user_identifier.clone())
                .collect();
            if dev_uids.is_empty() {
                return Err(CognitoIdentityError::ResourceNotFound {
                    resource_id: iid.to_string(),
                });
            }
            return Ok((iid.to_string(), dev_uids));
        }

        Err(CognitoIdentityError::ResourceNotFound {
            resource_id: "no identifier provided".to_string(),
        })
    }

    pub fn merge_developer_identities(
        &mut self,
        pool_id: &str,
        source_user_identifier: &str,
        destination_user_identifier: &str,
        developer_provider_name: &str,
        region: &str,
    ) -> Result<String, CognitoIdentityError> {
        if !self.identity_pools.contains_key(pool_id) {
            return Err(CognitoIdentityError::ResourceNotFound {
                resource_id: pool_id.to_string(),
            });
        }

        let source_key = Self::dev_identity_key(pool_id, source_user_identifier);
        let dest_key = Self::dev_identity_key(pool_id, destination_user_identifier);

        // Get or create the destination identity
        let dest_identity_id = if let Some(link) = self.developer_identities.get(&dest_key) {
            link.identity_id.clone()
        } else {
            let new_id = format!("{}:{}", region, uuid::Uuid::new_v4());
            let link = crate::types::DeveloperIdentityLink {
                identity_id: new_id.clone(),
                identity_pool_id: pool_id.to_string(),
                developer_provider_name: developer_provider_name.to_string(),
                developer_user_identifier: destination_user_identifier.to_string(),
            };
            self.developer_identities.insert(dest_key.clone(), link);
            new_id
        };

        // Merge the source into the destination: remove the source link
        if self.developer_identities.contains_key(&source_key) {
            self.developer_identities.remove(&source_key);
        }

        Ok(dest_identity_id)
    }

    pub fn unlink_developer_identity(
        &mut self,
        pool_id: &str,
        identity_id: &str,
        developer_provider_name: &str,
        developer_user_identifier: &str,
    ) -> Result<(), CognitoIdentityError> {
        if !self.identity_pools.contains_key(pool_id) {
            return Err(CognitoIdentityError::ResourceNotFound {
                resource_id: pool_id.to_string(),
            });
        }

        let key = Self::dev_identity_key(pool_id, developer_user_identifier);
        if let Some(link) = self.developer_identities.get(&key) {
            if link.identity_id == identity_id
                && link.developer_provider_name == developer_provider_name
            {
                self.developer_identities.remove(&key);
            }
        }
        Ok(())
    }

    pub fn unlink_identity(
        &mut self,
        identity_id: &str,
        logins: &HashMap<String, String>,
    ) -> Result<(), CognitoIdentityError> {
        // Remove any developer identity links for this identity
        let keys_to_remove: Vec<String> = self
            .developer_identities
            .iter()
            .filter(|(_, link)| link.identity_id == identity_id)
            .filter(|(_, link)| logins.contains_key(&link.developer_provider_name))
            .map(|(key, _)| key.clone())
            .collect();
        for key in keys_to_remove {
            self.developer_identities.remove(&key);
        }

        // Update the identity's logins list if it exists
        if let Some(pool_id) = self.identity_index.get(identity_id) {
            let pool_id = pool_id.clone();
            if let Some(identities) = self.identities.get_mut(&pool_id) {
                if let Some(identity) = identities.iter_mut().find(|i| i.identity_id == identity_id)
                {
                    identity.logins.retain(|login| !logins.contains_key(login));
                    identity.last_modified_date = Utc::now();
                }
            }
        }
        Ok(())
    }
}

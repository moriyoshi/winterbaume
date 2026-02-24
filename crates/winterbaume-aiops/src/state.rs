use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct AIOpsState {
    /// Investigation groups keyed by name.
    pub investigation_groups: HashMap<String, InvestigationGroup>,
}

#[derive(Debug, Error)]
pub enum AIOpsError {
    #[error("Investigation group {name} not found")]
    NotFound { name: String },

    #[error("Investigation group {name} already exists")]
    AlreadyExists { name: String },

    #[error("Investigation group resource not found: {arn}")]
    ResourceNotFound { arn: String },

    #[error("Resource policy not found for investigation group {name}")]
    PolicyNotFound { name: String },

    #[error("{message}")]
    Validation { message: String },
}

impl AIOpsState {
    /// Look up an investigation group by either its name or its ARN.
    pub fn lookup_name<'a>(&self, identifier: &'a str) -> Option<&'a str> {
        if !identifier.starts_with("arn:") {
            return Some(identifier);
        }
        // ARN format: arn:aws:aiops:<region>:<account>:investigation-group/<name>
        identifier.rsplit_once('/').map(|(_, name)| name)
    }

    pub fn create_investigation_group(
        &mut self,
        group: InvestigationGroup,
    ) -> Result<&InvestigationGroup, AIOpsError> {
        if self.investigation_groups.contains_key(&group.name) {
            return Err(AIOpsError::AlreadyExists {
                name: group.name.clone(),
            });
        }
        let name = group.name.clone();
        self.investigation_groups.insert(name.clone(), group);
        Ok(self.investigation_groups.get(&name).unwrap())
    }

    pub fn get_investigation_group(
        &self,
        identifier: &str,
    ) -> Result<&InvestigationGroup, AIOpsError> {
        let name = self
            .lookup_name(identifier)
            .ok_or_else(|| AIOpsError::NotFound {
                name: identifier.to_string(),
            })?;
        self.investigation_groups
            .get(name)
            .ok_or_else(|| AIOpsError::NotFound {
                name: name.to_string(),
            })
    }

    pub fn update_investigation_group(
        &mut self,
        identifier: &str,
        update: impl FnOnce(&mut InvestigationGroup),
    ) -> Result<&InvestigationGroup, AIOpsError> {
        let name = self
            .lookup_name(identifier)
            .ok_or_else(|| AIOpsError::NotFound {
                name: identifier.to_string(),
            })?
            .to_string();
        let group = self
            .investigation_groups
            .get_mut(&name)
            .ok_or_else(|| AIOpsError::NotFound { name: name.clone() })?;
        update(group);
        Ok(group)
    }

    pub fn delete_investigation_group(&mut self, identifier: &str) -> Result<(), AIOpsError> {
        let name = self
            .lookup_name(identifier)
            .ok_or_else(|| AIOpsError::NotFound {
                name: identifier.to_string(),
            })?
            .to_string();
        self.investigation_groups
            .remove(&name)
            .ok_or(AIOpsError::NotFound { name })?;
        Ok(())
    }

    pub fn list_investigation_groups(&self) -> Vec<&InvestigationGroup> {
        let mut groups: Vec<&InvestigationGroup> = self.investigation_groups.values().collect();
        groups.sort_by(|a, b| a.name.cmp(&b.name));
        groups
    }

    pub fn put_policy(&mut self, identifier: &str, policy: String) -> Result<String, AIOpsError> {
        let group = self.update_investigation_group(identifier, |g| g.policy = Some(policy))?;
        Ok(group.arn.clone())
    }

    pub fn get_policy(&self, identifier: &str) -> Result<(String, String), AIOpsError> {
        let group = self.get_investigation_group(identifier)?;
        let policy = group
            .policy
            .clone()
            .ok_or_else(|| AIOpsError::PolicyNotFound {
                name: group.name.clone(),
            })?;
        Ok((group.arn.clone(), policy))
    }

    pub fn delete_policy(&mut self, identifier: &str) -> Result<(), AIOpsError> {
        self.update_investigation_group(identifier, |g| g.policy = None)?;
        Ok(())
    }

    pub fn group_by_arn(&self, arn: &str) -> Result<&InvestigationGroup, AIOpsError> {
        self.investigation_groups
            .values()
            .find(|g| g.arn == arn)
            .ok_or_else(|| AIOpsError::ResourceNotFound {
                arn: arn.to_string(),
            })
    }

    pub fn group_by_arn_mut(&mut self, arn: &str) -> Result<&mut InvestigationGroup, AIOpsError> {
        self.investigation_groups
            .values_mut()
            .find(|g| g.arn == arn)
            .ok_or_else(|| AIOpsError::ResourceNotFound {
                arn: arn.to_string(),
            })
    }

    pub fn tag_resource(
        &mut self,
        arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), AIOpsError> {
        let group = self.group_by_arn_mut(arn)?;
        for (k, v) in tags {
            group.tags.insert(k, v);
        }
        Ok(())
    }

    pub fn untag_resource(&mut self, arn: &str, keys: &[String]) -> Result<(), AIOpsError> {
        let group = self.group_by_arn_mut(arn)?;
        for k in keys {
            group.tags.remove(k);
        }
        Ok(())
    }

    pub fn list_tags(&self, arn: &str) -> Result<HashMap<String, String>, AIOpsError> {
        Ok(self.group_by_arn(arn)?.tags.clone())
    }
}

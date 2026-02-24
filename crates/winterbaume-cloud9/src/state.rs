use std::collections::HashMap;

use thiserror::Error;

#[derive(Debug, Default)]
pub struct Cloud9State {
    pub environments: HashMap<String, EnvironmentRecord>,
    /// Memberships keyed by (environment_id, user_arn).
    pub memberships: HashMap<(String, String), MembershipRecord>,
    /// Tags keyed by environment ARN.
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone)]
pub struct EnvironmentRecord {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub description: Option<String>,
    pub env_type: String,
    pub connection_type: Option<String>,
    pub owner_arn: String,
    pub instance_type: Option<String>,
    pub image_id: Option<String>,
    pub managed_credentials_status: String,
    pub status: String,
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MembershipRecord {
    pub environment_id: String,
    pub user_arn: String,
    pub user_id: String,
    pub permissions: String,
    pub last_access: Option<f64>,
}

#[derive(Debug, Error)]
pub enum Cloud9Error {
    #[error("Environment {id} not found.")]
    EnvironmentNotFound { id: String },

    #[error("Membership for env {environment_id} user {user_arn} not found.")]
    MembershipNotFound {
        environment_id: String,
        user_arn: String,
    },

    #[error("{message}")]
    Validation { message: String },
}

impl Cloud9State {
    pub fn create_environment(&mut self, env: EnvironmentRecord) -> &EnvironmentRecord {
        let id = env.id.clone();
        self.environments.insert(id.clone(), env);
        self.environments.get(&id).unwrap()
    }

    pub fn get_environment(&self, id: &str) -> Result<&EnvironmentRecord, Cloud9Error> {
        self.environments
            .get(id)
            .ok_or(Cloud9Error::EnvironmentNotFound { id: id.to_string() })
    }

    pub fn delete_environment(&mut self, id: &str) -> Result<(), Cloud9Error> {
        self.environments
            .remove(id)
            .ok_or(Cloud9Error::EnvironmentNotFound { id: id.to_string() })?;
        // Cascade memberships.
        self.memberships.retain(|(env_id, _), _| env_id != id);
        Ok(())
    }

    pub fn update_environment(
        &mut self,
        id: &str,
        update: impl FnOnce(&mut EnvironmentRecord),
    ) -> Result<&EnvironmentRecord, Cloud9Error> {
        let env = self
            .environments
            .get_mut(id)
            .ok_or(Cloud9Error::EnvironmentNotFound { id: id.to_string() })?;
        update(env);
        Ok(env)
    }

    pub fn list_environment_ids(&self) -> Vec<String> {
        let mut v: Vec<String> = self.environments.keys().cloned().collect();
        v.sort();
        v
    }

    pub fn describe_environments<'a>(&'a self, ids: &'a [String]) -> Vec<&'a EnvironmentRecord> {
        ids.iter()
            .filter_map(|id| self.environments.get(id))
            .collect()
    }

    pub fn upsert_membership(&mut self, m: MembershipRecord) -> &MembershipRecord {
        let key = (m.environment_id.clone(), m.user_arn.clone());
        self.memberships.insert(key.clone(), m);
        self.memberships.get(&key).unwrap()
    }

    pub fn delete_membership(
        &mut self,
        environment_id: &str,
        user_arn: &str,
    ) -> Result<(), Cloud9Error> {
        self.memberships
            .remove(&(environment_id.to_string(), user_arn.to_string()))
            .ok_or(Cloud9Error::MembershipNotFound {
                environment_id: environment_id.to_string(),
                user_arn: user_arn.to_string(),
            })?;
        Ok(())
    }

    pub fn update_membership(
        &mut self,
        environment_id: &str,
        user_arn: &str,
        permissions: String,
    ) -> Result<&MembershipRecord, Cloud9Error> {
        let m = self
            .memberships
            .get_mut(&(environment_id.to_string(), user_arn.to_string()))
            .ok_or(Cloud9Error::MembershipNotFound {
                environment_id: environment_id.to_string(),
                user_arn: user_arn.to_string(),
            })?;
        m.permissions = permissions;
        Ok(m)
    }

    pub fn describe_memberships(
        &self,
        environment_id: Option<&str>,
        user_arn: Option<&str>,
        permissions: Option<&[String]>,
    ) -> Vec<&MembershipRecord> {
        let mut v: Vec<&MembershipRecord> = self
            .memberships
            .values()
            .filter(|m| environment_id.is_none_or(|id| id == m.environment_id))
            .filter(|m| user_arn.is_none_or(|u| u == m.user_arn))
            .filter(|m| permissions.is_none_or(|p| p.iter().any(|pp| pp == &m.permissions)))
            .collect();
        v.sort_by(|a, b| {
            (a.environment_id.as_str(), a.user_arn.as_str())
                .cmp(&(b.environment_id.as_str(), b.user_arn.as_str()))
        });
        v
    }

    pub fn tag_resource(&mut self, arn: &str, tags: HashMap<String, String>) {
        let entry = self.tags.entry(arn.to_string()).or_default();
        for (k, v) in tags {
            entry.insert(k, v);
        }
    }

    pub fn untag_resource(&mut self, arn: &str, keys: &[String]) {
        if let Some(entry) = self.tags.get_mut(arn) {
            for k in keys {
                entry.remove(k);
            }
        }
    }

    pub fn list_tags(&self, arn: &str) -> HashMap<String, String> {
        self.tags.get(arn).cloned().unwrap_or_default()
    }
}

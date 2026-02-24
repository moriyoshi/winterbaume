use std::collections::HashMap;

use thiserror::Error;

use crate::model;

#[derive(Debug, Default)]
pub struct DlmState {
    pub policies: HashMap<String, model::LifecyclePolicy>,
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Error)]
pub enum DlmError {
    #[error("LifecyclePolicy {id} does not exist.")]
    PolicyNotFound { id: String },

    #[error("Resource {arn} does not exist.")]
    ResourceNotFound { arn: String },

    #[error("{message}")]
    Validation { message: String },
}

impl DlmState {
    pub fn create_policy(&mut self, policy: model::LifecyclePolicy) -> &model::LifecyclePolicy {
        let id = policy.policy_id.clone().unwrap_or_default();
        self.policies.insert(id.clone(), policy);
        self.policies.get(&id).unwrap()
    }

    pub fn get_policy(&self, id: &str) -> Result<&model::LifecyclePolicy, DlmError> {
        self.policies
            .get(id)
            .ok_or(DlmError::PolicyNotFound { id: id.to_string() })
    }

    pub fn update_policy(
        &mut self,
        id: &str,
        update: impl FnOnce(&mut model::LifecyclePolicy),
    ) -> Result<&model::LifecyclePolicy, DlmError> {
        let p = self
            .policies
            .get_mut(id)
            .ok_or(DlmError::PolicyNotFound { id: id.to_string() })?;
        update(p);
        p.date_modified = Some(chrono::Utc::now().to_rfc3339());
        Ok(p)
    }

    pub fn delete_policy(&mut self, id: &str) -> Result<(), DlmError> {
        self.policies
            .remove(id)
            .ok_or(DlmError::PolicyNotFound { id: id.to_string() })?;
        Ok(())
    }

    pub fn list_policies(&self) -> Vec<&model::LifecyclePolicy> {
        let mut items: Vec<&model::LifecyclePolicy> = self.policies.values().collect();
        items.sort_by(|a, b| a.policy_id.cmp(&b.policy_id));
        items
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

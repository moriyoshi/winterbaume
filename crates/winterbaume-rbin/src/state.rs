use std::collections::HashMap;

use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct RbinState {
    pub rules: HashMap<String, RuleRecord>,
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone)]
pub struct RuleRecord {
    pub identifier: String,
    pub description: Option<String>,
    pub resource_type: String,
    pub retention_period: Option<Value>,
    pub resource_tags: Vec<Value>,
    pub exclude_resource_tags: Vec<Value>,
    pub status: String,
    pub lock_state: Option<String>,
    pub lock_configuration: Option<Value>,
    pub lock_end_time: Option<f64>,
    pub rule_arn: String,
}

#[derive(Debug, Error)]
pub enum RbinError {
    #[error("Rule {id} does not exist.")]
    NotFound { id: String },

    #[error("Rule {id} is in lock state {state}.")]
    LockStateError { id: String, state: String },

    #[error("{message}")]
    Validation { message: String },
}

impl RbinState {
    pub fn create_rule(&mut self, rule: RuleRecord) -> &RuleRecord {
        let id = rule.identifier.clone();
        self.rules.insert(id.clone(), rule);
        self.rules.get(&id).unwrap()
    }

    pub fn get_rule(&self, id: &str) -> Result<&RuleRecord, RbinError> {
        self.rules
            .get(id)
            .ok_or(RbinError::NotFound { id: id.to_string() })
    }

    pub fn update_rule(
        &mut self,
        id: &str,
        update: impl FnOnce(&mut RuleRecord),
    ) -> Result<&RuleRecord, RbinError> {
        let r = self
            .rules
            .get_mut(id)
            .ok_or(RbinError::NotFound { id: id.to_string() })?;
        update(r);
        Ok(r)
    }

    pub fn delete_rule(&mut self, id: &str) -> Result<(), RbinError> {
        // Cannot delete locked rules.
        if let Some(r) = self.rules.get(id) {
            if r.lock_state.as_deref() == Some("locked") {
                return Err(RbinError::LockStateError {
                    id: id.to_string(),
                    state: "locked".to_string(),
                });
            }
        }
        self.rules
            .remove(id)
            .ok_or(RbinError::NotFound { id: id.to_string() })?;
        Ok(())
    }

    pub fn list_rules(&self, resource_type: Option<&str>) -> Vec<&RuleRecord> {
        let mut items: Vec<&RuleRecord> = self
            .rules
            .values()
            .filter(|r| resource_type.is_none_or(|t| r.resource_type == t))
            .collect();
        items.sort_by(|a, b| a.identifier.cmp(&b.identifier));
        items
    }

    pub fn lock_rule(&mut self, id: &str, config: Value) -> Result<&RuleRecord, RbinError> {
        let r = self
            .rules
            .get_mut(id)
            .ok_or(RbinError::NotFound { id: id.to_string() })?;
        r.lock_state = Some("locked".to_string());
        r.lock_configuration = Some(config);
        Ok(r)
    }

    pub fn unlock_rule(&mut self, id: &str) -> Result<&RuleRecord, RbinError> {
        let r = self
            .rules
            .get_mut(id)
            .ok_or(RbinError::NotFound { id: id.to_string() })?;
        if r.lock_state.as_deref() == Some("locked") {
            r.lock_state = Some("pending_unlock".to_string());
            r.lock_end_time = Some(chrono::Utc::now().timestamp() as f64 + 7.0 * 86400.0);
        }
        Ok(r)
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

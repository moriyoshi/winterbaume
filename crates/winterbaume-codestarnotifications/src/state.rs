use std::collections::HashMap;

use thiserror::Error;

#[derive(Debug, Default)]
pub struct CodeStarNotificationsState {
    pub rules: HashMap<String, RuleRecord>,
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone)]
pub struct RuleRecord {
    pub arn: String,
    pub name: String,
    pub resource: String,
    pub detail_type: String,
    pub event_type_ids: Vec<String>,
    pub status: String,
    pub targets: Vec<TargetRecord>,
    pub created_timestamp: f64,
    pub last_modified_timestamp: f64,
}

#[derive(Debug, Clone)]
pub struct TargetRecord {
    pub target_address: String,
    pub target_type: String,
    pub target_status: String,
}

#[derive(Debug, Error)]
pub enum CodeStarError {
    #[error("Notification rule {arn} does not exist.")]
    NotFound { arn: String },

    #[error("{message}")]
    Validation { message: String },
}

impl CodeStarNotificationsState {
    pub fn put_rule(&mut self, rule: RuleRecord) {
        self.rules.insert(rule.arn.clone(), rule);
    }

    pub fn list_rules(&self) -> Vec<&RuleRecord> {
        let mut items: Vec<&RuleRecord> = self.rules.values().collect();
        items.sort_by(|a, b| a.arn.cmp(&b.arn));
        items
    }

    pub fn list_targets(&self) -> Vec<TargetRecord> {
        let mut seen: HashMap<String, TargetRecord> = HashMap::new();
        for rule in self.rules.values() {
            for t in &rule.targets {
                seen.entry(t.target_address.clone())
                    .or_insert_with(|| t.clone());
            }
        }
        let mut out: Vec<TargetRecord> = seen.into_values().collect();
        out.sort_by(|a, b| a.target_address.cmp(&b.target_address));
        out
    }

    pub fn delete_target(&mut self, address: &str, force_unsubscribe_all: bool) {
        for rule in self.rules.values_mut() {
            if force_unsubscribe_all {
                rule.targets.retain(|t| t.target_address != address);
            } else if rule.targets.iter().any(|t| t.target_address == address) {
                // when not forcing, leave the rule's targets alone.
            }
        }
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

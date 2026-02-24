use std::collections::HashMap;

use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct SsmQuickSetupState {
    /// Configuration managers keyed by ARN.
    pub managers: HashMap<String, ManagerRecord>,
    /// Tags keyed by manager ARN.
    pub tags: HashMap<String, HashMap<String, String>>,
    /// Service settings (single per account-region).
    pub service_settings: ServiceSettingsRecord,
}

#[derive(Debug, Clone)]
pub struct ManagerRecord {
    pub arn: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
    pub last_modified_at: String,
    pub status_summary: String,
    /// Configuration definitions stored opaquely as wire JSON, each augmented with
    /// an `Id` field synthesised on insert.
    pub definitions: Vec<Value>,
}

#[derive(Debug, Clone, Default)]
pub struct ServiceSettingsRecord {
    pub explorer_enabling_role_arn: Option<String>,
}

#[derive(Debug, Error)]
pub enum SsmQuickSetupError {
    #[error("Configuration manager {arn} not found.")]
    ManagerNotFound { arn: String },

    #[error("Configuration definition {id} not found in manager {arn}.")]
    DefinitionNotFound { arn: String, id: String },

    #[error("{message}")]
    Validation { message: String },
}

impl SsmQuickSetupState {
    pub fn create_manager(&mut self, record: ManagerRecord) -> &ManagerRecord {
        let arn = record.arn.clone();
        self.managers.insert(arn.clone(), record);
        self.managers.get(&arn).unwrap()
    }

    pub fn get_manager(&self, arn: &str) -> Result<&ManagerRecord, SsmQuickSetupError> {
        self.managers
            .get(arn)
            .ok_or(SsmQuickSetupError::ManagerNotFound {
                arn: arn.to_string(),
            })
    }

    pub fn get_manager_mut(&mut self, arn: &str) -> Result<&mut ManagerRecord, SsmQuickSetupError> {
        self.managers
            .get_mut(arn)
            .ok_or(SsmQuickSetupError::ManagerNotFound {
                arn: arn.to_string(),
            })
    }

    pub fn delete_manager(&mut self, arn: &str) -> Result<(), SsmQuickSetupError> {
        self.managers
            .remove(arn)
            .ok_or(SsmQuickSetupError::ManagerNotFound {
                arn: arn.to_string(),
            })?;
        self.tags.remove(arn);
        Ok(())
    }

    pub fn list_managers(&self) -> Vec<&ManagerRecord> {
        let mut v: Vec<&ManagerRecord> = self.managers.values().collect();
        v.sort_by(|a, b| a.arn.cmp(&b.arn));
        v
    }

    pub fn find_definition_arn(&self, definition_id: &str) -> Option<(&str, &Value)> {
        for m in self.managers.values() {
            for def in &m.definitions {
                if def.get("Id").and_then(|v| v.as_str()) == Some(definition_id) {
                    return Some((m.arn.as_str(), def));
                }
            }
        }
        None
    }

    pub fn list_all_definitions(&self) -> Vec<&Value> {
        let mut v: Vec<&Value> = self
            .managers
            .values()
            .flat_map(|m| m.definitions.iter())
            .collect();
        v.sort_by(|a, b| {
            a.get("Id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .cmp(b.get("Id").and_then(|v| v.as_str()).unwrap_or(""))
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

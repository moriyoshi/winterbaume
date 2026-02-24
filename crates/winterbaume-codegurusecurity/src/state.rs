use std::collections::HashMap;

use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct CodeGuruSecurityState {
    pub scans: HashMap<String, ScanRecord>,
    pub findings: HashMap<String, Vec<Value>>,
    pub account_configuration: Value,
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Default)]
pub struct ScanRecord {
    pub scan: Value,
}

#[derive(Debug, Error)]
pub enum CodeGuruSecurityError {
    #[error("Scan {name} does not exist.")]
    NotFound { name: String },

    #[error("{message}")]
    Validation { message: String },
}

impl CodeGuruSecurityState {
    pub fn put_scan(&mut self, name: String, scan: Value) {
        self.scans.insert(name, ScanRecord { scan });
    }

    pub fn list_scans(&self) -> Vec<&Value> {
        let mut items: Vec<&Value> = self.scans.values().map(|r| &r.scan).collect();
        items.sort_by(|a, b| {
            a.get("scanName")
                .and_then(|v| v.as_str())
                .cmp(&b.get("scanName").and_then(|v| v.as_str()))
        });
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

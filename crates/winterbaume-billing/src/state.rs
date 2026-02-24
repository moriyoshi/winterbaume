use std::collections::HashMap;

use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct BillingState {
    pub billing_views: HashMap<String, Value>,
    pub tags: HashMap<String, HashMap<String, String>>,
    pub policies: HashMap<String, String>,
}

#[derive(Debug, Error)]
pub enum BillingError {
    #[error("Billing view {arn} does not exist.")]
    NotFound { arn: String },

    #[error("{message}")]
    Validation { message: String },
}

impl BillingState {
    pub fn put_view(&mut self, arn: String, view: Value) {
        self.billing_views.insert(arn, view);
    }

    pub fn list_views(&self) -> Vec<&Value> {
        let mut items: Vec<&Value> = self.billing_views.values().collect();
        items.sort_by(|a, b| {
            a.get("arn")
                .and_then(|v| v.as_str())
                .cmp(&b.get("arn").and_then(|v| v.as_str()))
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

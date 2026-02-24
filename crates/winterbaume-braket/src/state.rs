use std::collections::HashMap;

use serde_json::{Value, json};
use thiserror::Error;

#[derive(Debug, Default)]
pub struct BraketState {
    pub jobs: HashMap<String, Value>,
    pub tasks: HashMap<String, Value>,
    pub spending_limits: HashMap<String, Value>,
    pub devices: HashMap<String, Value>,
    pub tags: HashMap<String, HashMap<String, String>>,
    seeded: bool,
}

#[derive(Debug, Error)]
pub enum BraketError {
    #[error("Resource {arn} does not exist.")]
    NotFound { arn: String },

    #[error("{message}")]
    Validation { message: String },
}

impl BraketState {
    pub fn ensure_seeded(&mut self) {
        if self.seeded {
            return;
        }
        self.seeded = true;
        let arn = "arn:aws:braket:::device/quantum-simulator/amazon/sv1".to_string();
        self.devices.insert(
            arn.clone(),
            json!({
                "deviceArn": arn,
                "deviceName": "SV1",
                "deviceType": "SIMULATOR",
                "deviceStatus": "ONLINE",
                "providerName": "Amazon Braket",
                "deviceCapabilities": "{}",
                "deviceQueueInfo": [],
            }),
        );
    }

    pub fn list_devices(&self) -> Vec<&Value> {
        let mut items: Vec<&Value> = self.devices.values().collect();
        items.sort_by(|a, b| {
            a.get("deviceArn")
                .and_then(|v| v.as_str())
                .cmp(&b.get("deviceArn").and_then(|v| v.as_str()))
        });
        items
    }

    pub fn list_jobs(&self) -> Vec<&Value> {
        let mut items: Vec<&Value> = self.jobs.values().collect();
        items.sort_by(|a, b| {
            a.get("jobArn")
                .and_then(|v| v.as_str())
                .cmp(&b.get("jobArn").and_then(|v| v.as_str()))
        });
        items
    }

    pub fn list_tasks(&self) -> Vec<&Value> {
        let mut items: Vec<&Value> = self.tasks.values().collect();
        items.sort_by(|a, b| {
            a.get("quantumTaskArn")
                .and_then(|v| v.as_str())
                .cmp(&b.get("quantumTaskArn").and_then(|v| v.as_str()))
        });
        items
    }

    pub fn list_spending_limits(&self) -> Vec<&Value> {
        let mut items: Vec<&Value> = self.spending_limits.values().collect();
        items.sort_by(|a, b| {
            a.get("spendingLimitArn")
                .and_then(|v| v.as_str())
                .cmp(&b.get("spendingLimitArn").and_then(|v| v.as_str()))
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

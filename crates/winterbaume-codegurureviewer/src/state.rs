use std::collections::HashMap;

use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct CodeGuruReviewerState {
    pub associations: HashMap<String, Value>,
    pub code_reviews: HashMap<String, Value>,
    pub recommendations: HashMap<String, Vec<Value>>,
    pub feedback: HashMap<FeedbackKey, Value>,
    pub tags: HashMap<String, HashMap<String, String>>,
}

pub type FeedbackKey = (String, String, String);

#[derive(Debug, Error)]
pub enum CodeGuruReviewerError {
    #[error("Resource {0} does not exist.")]
    NotFound(String),

    #[error("{0}")]
    Validation(String),
}

impl CodeGuruReviewerState {
    pub fn put_association(&mut self, arn: String, value: Value) {
        self.associations.insert(arn, value);
    }

    pub fn list_associations(&self) -> Vec<&Value> {
        let mut items: Vec<&Value> = self.associations.values().collect();
        items.sort_by(|a, b| {
            a.get("AssociationArn")
                .and_then(|v| v.as_str())
                .cmp(&b.get("AssociationArn").and_then(|v| v.as_str()))
        });
        items
    }

    pub fn list_code_reviews(&self) -> Vec<&Value> {
        let mut items: Vec<&Value> = self.code_reviews.values().collect();
        items.sort_by(|a, b| {
            a.get("CodeReviewArn")
                .and_then(|v| v.as_str())
                .cmp(&b.get("CodeReviewArn").and_then(|v| v.as_str()))
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

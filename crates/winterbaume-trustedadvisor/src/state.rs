use std::collections::HashMap;

use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct TrustedAdvisorState {
    /// Recommendations keyed by identifier (id or arn).
    pub recommendations: HashMap<String, Value>,
    /// Organization recommendations keyed by identifier.
    pub organization_recommendations: HashMap<String, Value>,
    /// Resource exclusions keyed by ARN -> bool.
    pub resource_exclusions: HashMap<String, bool>,
}

#[derive(Debug, Error)]
pub enum TrustedAdvisorError {
    #[error("Recommendation {id} does not exist.")]
    NotFound { id: String },

    #[error("{message}")]
    Validation { message: String },
}

impl TrustedAdvisorState {
    pub fn upsert_recommendation(&mut self, id: &str, value: Value) {
        self.recommendations.insert(id.to_string(), value);
    }

    pub fn upsert_organization_recommendation(&mut self, id: &str, value: Value) {
        self.organization_recommendations
            .insert(id.to_string(), value);
    }

    pub fn get_recommendation(&self, id: &str) -> Option<&Value> {
        self.recommendations.get(id)
    }

    pub fn get_organization_recommendation(&self, id: &str) -> Option<&Value> {
        self.organization_recommendations.get(id)
    }

    pub fn list_recommendations(&self) -> Vec<&Value> {
        let mut v: Vec<&Value> = self.recommendations.values().collect();
        v.sort_by(|a, b| {
            let ka = a.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let kb = b.get("id").and_then(|v| v.as_str()).unwrap_or("");
            ka.cmp(kb)
        });
        v
    }

    pub fn list_organization_recommendations(&self) -> Vec<&Value> {
        let mut v: Vec<&Value> = self.organization_recommendations.values().collect();
        v.sort_by(|a, b| {
            let ka = a.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let kb = b.get("id").and_then(|v| v.as_str()).unwrap_or("");
            ka.cmp(kb)
        });
        v
    }

    pub fn update_recommendation_lifecycle(
        &mut self,
        id: &str,
        stage: &str,
    ) -> Result<(), TrustedAdvisorError> {
        let entry = self
            .recommendations
            .get_mut(id)
            .ok_or(TrustedAdvisorError::NotFound { id: id.to_string() })?;
        if let Some(obj) = entry.as_object_mut() {
            obj.insert(
                "lifecycleStage".to_string(),
                Value::String(stage.to_string()),
            );
        }
        Ok(())
    }

    pub fn update_organization_recommendation_lifecycle(
        &mut self,
        id: &str,
        stage: &str,
    ) -> Result<(), TrustedAdvisorError> {
        let entry = self
            .organization_recommendations
            .get_mut(id)
            .ok_or(TrustedAdvisorError::NotFound { id: id.to_string() })?;
        if let Some(obj) = entry.as_object_mut() {
            obj.insert(
                "lifecycleStage".to_string(),
                Value::String(stage.to_string()),
            );
        }
        Ok(())
    }

    pub fn set_resource_exclusion(&mut self, arn: &str, excluded: bool) {
        self.resource_exclusions.insert(arn.to_string(), excluded);
    }
}

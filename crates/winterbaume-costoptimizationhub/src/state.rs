use std::collections::HashMap;

use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CostOptimizationHubError {
    #[error("Recommendation {0} not found")]
    RecommendationNotFound(String),
}

#[derive(Debug, Default)]
pub struct CostOptimizationHubState {
    pub enrollment_statuses: HashMap<String, Value>,
    pub preferences: Value,
    pub recommendations: HashMap<String, Value>,
}

impl CostOptimizationHubState {
    pub fn list_enrollment_statuses(&self, include_org: bool) -> Vec<Value> {
        let mut items: Vec<Value> = self.enrollment_statuses.values().cloned().collect();
        items.sort_by(|a, b| {
            a.get("accountId")
                .and_then(|v| v.as_str())
                .cmp(&b.get("accountId").and_then(|v| v.as_str()))
        });
        if !include_org {
            items.truncate(items.len().min(100));
        }
        items
    }

    pub fn get_recommendation(&self, id: &str) -> Result<Value, CostOptimizationHubError> {
        self.recommendations
            .get(id)
            .cloned()
            .ok_or_else(|| CostOptimizationHubError::RecommendationNotFound(id.to_string()))
    }

    pub fn list_recommendations(&self) -> Vec<Value> {
        let mut items: Vec<Value> = self.recommendations.values().cloned().collect();
        items.sort_by(|a, b| {
            a.get("recommendationId")
                .and_then(|v| v.as_str())
                .cmp(&b.get("recommendationId").and_then(|v| v.as_str()))
        });
        items
    }
}

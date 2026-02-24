//! Domain types for the Cost Explorer service.

use serde::{Deserialize, Serialize};

/// A Cost Category definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostCategoryDefinition {
    pub name: String,
    pub cost_category_arn: String,
    pub effective_start: String,
    pub rule_version: String,
    pub rules: Vec<CostCategoryRule>,
}

/// A single rule within a Cost Category definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostCategoryRule {
    pub value: Option<String>,
    pub rule: Option<serde_json::Value>,
}

/// An anomaly monitor record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyMonitorRecord {
    pub monitor_arn: String,
    pub monitor_name: String,
    pub monitor_type: String,
    pub monitor_dimension: Option<String>,
    pub creation_date: String,
    pub last_updated_date: String,
    pub last_evaluated_date: Option<String>,
}

/// An anomaly subscription record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalySubscriptionRecord {
    pub subscription_arn: String,
    pub subscription_name: String,
    pub account_id: String,
    pub monitor_arn_list: Vec<String>,
    pub subscribers: Vec<SubscriberRecord>,
    pub frequency: String,
    pub threshold: Option<f64>,
}

/// A subscriber within an anomaly subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriberRecord {
    pub address: Option<String>,
    pub status: Option<String>,
    pub subscriber_type: Option<String>,
}

/// A cost allocation tag record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostAllocationTagRecord {
    pub tag_key: String,
    pub status: String,
    pub tag_type: String,
    pub last_updated_date: Option<String>,
    pub last_used_date: Option<String>,
}

/// A cost allocation tag backfill job record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostAllocationTagBackfillRecord {
    pub backfill_from: String,
    pub backfill_status: String,
    pub requested_at: String,
    pub completed_at: Option<String>,
    pub last_updated_at: Option<String>,
}

/// Anomaly feedback record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyFeedbackRecord {
    pub anomaly_id: String,
    pub feedback: String,
}

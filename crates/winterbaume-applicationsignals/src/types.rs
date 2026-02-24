use std::collections::HashMap;

use serde_json::Value;

#[derive(Debug, Clone)]
pub struct ServiceLevelObjective {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub description: Option<String>,
    pub evaluation_type: String,
    pub metric_source_type: String,
    pub created_time: i64,
    pub last_updated_time: i64,
    pub sli: Option<Value>,
    pub request_based_sli: Option<Value>,
    pub goal: Option<Value>,
    pub burn_rate_configurations: Vec<Value>,
    pub exclusion_windows: Vec<Value>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Default)]
pub struct GroupingConfiguration {
    pub grouping_attribute_definitions: Vec<Value>,
    pub updated_at: i64,
}

#[derive(Debug, Clone)]
pub struct ServiceEntry {
    pub key_attributes: HashMap<String, String>,
    pub attribute_maps: Vec<HashMap<String, String>>,
    pub metric_references: Vec<Value>,
    pub log_group_references: Vec<HashMap<String, String>>,
    pub service_groups: Vec<Value>,
}

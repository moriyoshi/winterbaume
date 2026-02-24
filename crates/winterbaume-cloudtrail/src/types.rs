use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// An event data store.
#[derive(Debug, Clone)]
pub struct EventDataStoreData {
    pub event_data_store_arn: String,
    pub name: String,
    pub status: String,
    pub multi_region_enabled: bool,
    pub organization_enabled: bool,
    pub retention_period: i32,
    pub termination_protection_enabled: bool,
    pub created_timestamp: DateTime<Utc>,
    pub updated_timestamp: DateTime<Utc>,
    pub tags: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct Trail {
    pub name: String,
    pub s3_bucket_name: String,
    pub s3_key_prefix: String,
    pub include_global_service_events: bool,
    pub is_multi_region_trail: bool,
    pub trail_arn: String,
    pub home_region: String,
    pub is_logging: bool,
    pub latest_delivery_time: Option<DateTime<Utc>>,
    pub tags: HashMap<String, String>,
    pub event_selectors: Vec<EventSelector>,
    pub insight_selectors: Vec<InsightSelector>,
}

#[derive(Debug, Clone)]
pub struct EventSelector {
    pub read_write_type: String,
    pub include_management_events: bool,
    pub data_resources: Vec<DataResource>,
    pub exclude_management_event_sources: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DataResource {
    pub r#type: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct InsightSelector {
    pub insight_type: String,
}

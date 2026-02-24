use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// An OpenSearch Ingestion pipeline.
#[derive(Debug, Clone)]
pub struct Pipeline {
    pub pipeline_name: String,
    pub pipeline_arn: String,
    pub min_units: i32,
    pub max_units: i32,
    pub status: String,
    pub pipeline_configuration_body: String,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub ingest_endpoint_urls: Vec<String>,
    pub tags: HashMap<String, String>,
}

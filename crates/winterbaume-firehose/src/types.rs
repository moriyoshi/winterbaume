use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct DeliveryStream {
    pub name: String,
    pub arn: String,
    pub status: String,
    pub destination_type: String,
    pub destination_id: String,
    pub created_timestamp: DateTime<Utc>,
    pub records: Vec<FirehoseRecord>,
    pub tags: HashMap<String, String>,
    pub encryption_status: String,
    pub version_id: String,
}

#[derive(Debug, Clone)]
pub struct FirehoseRecord {
    pub record_id: String,
}

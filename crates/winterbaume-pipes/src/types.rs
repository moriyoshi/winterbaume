use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// An EventBridge Pipe.
#[derive(Debug, Clone)]
pub struct Pipe {
    pub name: String,
    pub arn: String,
    pub source: String,
    pub target: String,
    pub description: Option<String>,
    pub enrichment: Option<String>,
    pub role_arn: Option<String>,
    pub desired_state: String,
    pub current_state: String,
    pub creation_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

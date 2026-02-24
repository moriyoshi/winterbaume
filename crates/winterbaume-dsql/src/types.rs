use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// A DSQL cluster.
#[derive(Debug, Clone)]
pub struct Cluster {
    pub identifier: String,
    pub arn: String,
    pub status: String,
    pub creation_time: DateTime<Utc>,
    pub deletion_protection_enabled: bool,
    pub tags: HashMap<String, String>,
}

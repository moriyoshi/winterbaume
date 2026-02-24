use chrono::{DateTime, Utc};

/// A MediaPackage channel.
#[derive(Debug, Clone)]
pub struct Channel {
    pub arn: String,
    pub id: String,
    pub description: String,
    pub tags: std::collections::HashMap<String, String>,
    pub created_at: DateTime<Utc>,
}

/// A MediaPackage origin endpoint.
#[derive(Debug, Clone)]
pub struct OriginEndpoint {
    pub arn: String,
    pub id: String,
    pub channel_id: String,
    pub description: String,
    pub manifest_name: String,
    pub startover_window_seconds: i32,
    pub time_delay_seconds: i32,
    pub url: String,
    pub tags: std::collections::HashMap<String, String>,
}

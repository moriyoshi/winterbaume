use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct PinpointApp {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub creation_date: DateTime<Utc>,
    pub tags: HashMap<String, String>,
    pub settings: Option<ApplicationSettings>,
    pub event_stream: Option<EventStream>,
    /// Quiet time as `{"start": "HH:MM", "end": "HH:MM"}` — stored for Terraform round-trip.
    pub quiet_time: Option<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct ApplicationSettings {
    pub campaign_hook: Option<CampaignHook>,
    pub limits: Option<Limits>,
    pub last_modified_date: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CampaignHook {
    pub lambda_function_name: Option<String>,
    pub mode: Option<String>,
    pub web_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Limits {
    pub daily: Option<i64>,
    pub maximum_duration: Option<i64>,
    pub messages_per_second: Option<i64>,
    pub total: Option<i64>,
    pub session: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct EmailChannel {
    pub application_id: String,
    pub enabled: bool,
    pub from_address: String,
    pub identity: String,
    pub role_arn: Option<String>,
    pub configuration_set: Option<String>,
    pub messages_per_second: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct EventStream {
    pub application_id: String,
    pub destination_stream_arn: String,
    pub role_arn: String,
    pub last_modified_date: DateTime<Utc>,
}

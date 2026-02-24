use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MediaLiveChannel {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub state: String,
    pub channel_class: String,
    pub pipelines_running_count: i32,
    pub role_arn: String,
    pub input_attachments: serde_json::Value,
    pub destinations: serde_json::Value,
    pub encoder_settings: serde_json::Value,
    pub input_specification: serde_json::Value,
    pub log_level: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct MediaLiveInput {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub state: String,
    pub input_class: String,
    pub input_source_type: String,
    pub input_type: String,
    pub role_arn: String,
    pub attached_channels: Vec<String>,
    pub destinations: serde_json::Value,
    pub input_devices: serde_json::Value,
    pub media_connect_flows: serde_json::Value,
    pub security_groups: Vec<String>,
    pub sources: serde_json::Value,
    pub tags: HashMap<String, String>,
}

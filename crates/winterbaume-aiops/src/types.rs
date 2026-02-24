use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct InvestigationGroup {
    pub name: String,
    pub arn: String,
    pub role_arn: String,
    pub encryption_configuration: Option<EncryptionConfiguration>,
    pub retention_in_days: Option<i64>,
    pub chatbot_notification_channel: Option<HashMap<String, Vec<String>>>,
    pub tag_key_boundaries: Option<Vec<String>>,
    pub is_cloud_trail_event_history_enabled: Option<bool>,
    pub cross_account_configurations: Option<Vec<CrossAccountConfiguration>>,
    pub created_by: String,
    pub created_at: i64,
    pub last_modified_by: String,
    pub last_modified_at: i64,
    pub policy: Option<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct EncryptionConfiguration {
    pub r#type: Option<String>,
    pub kms_key_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CrossAccountConfiguration {
    pub source_role_arn: Option<String>,
}

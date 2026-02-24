use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ConnectInstance {
    pub id: String,
    pub arn: String,
    pub identity_management_type: String,
    pub instance_alias: Option<String>,
    pub instance_status: String,
    pub created_time: DateTime<Utc>,
    pub inbound_calls_enabled: bool,
    pub outbound_calls_enabled: bool,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct AnalyticsDataAssociation {
    pub instance_id: String,
    pub data_set_id: String,
    pub target_account_id: Option<String>,
    pub resource_share_id: String,
    pub resource_share_arn: String,
}

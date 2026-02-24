use std::collections::HashMap;

use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Application {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub namespace: String,
    pub description: Option<String>,
    pub application_type: Option<String>,
    pub client_token: Option<String>,
    pub initialization_timeout: Option<i32>,
    pub is_service: Option<bool>,
    pub permissions: Vec<String>,
    pub publications: Vec<Value>,
    pub subscriptions: Vec<Value>,
    pub application_config: Option<Value>,
    pub application_source_config: Option<Value>,
    pub iframe_config: Option<Value>,
    pub created_time: i64,
    pub last_modified_time: i64,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct DataIntegration {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub description: Option<String>,
    pub kms_key: String,
    pub source_uri: Option<String>,
    pub file_configuration: Option<Value>,
    pub object_configuration: Option<Value>,
    pub schedule_config: Option<Value>,
    pub client_token: Option<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct DataIntegrationAssociation {
    pub id: String,
    pub arn: String,
    pub data_integration_id: String,
    pub data_integration_arn: String,
    pub client_id: Option<String>,
    pub destination_uri: Option<String>,
    pub last_execution_status: Option<String>,
    pub execution_configuration: Option<Value>,
    pub client_association_metadata: HashMap<String, String>,
    pub created_time: i64,
    pub last_modified_time: i64,
}

#[derive(Debug, Clone)]
pub struct EventIntegration {
    pub arn: String,
    pub name: String,
    pub description: Option<String>,
    pub event_filter_source: String,
    pub event_bridge_bus: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct EventIntegrationAssociationRecord {
    pub event_integration_association_arn: String,
    pub event_integration_association_id: String,
    pub event_integration_name: String,
    pub client_id: String,
    pub event_bridge_rule_name: String,
    pub client_association_metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct ApplicationAssociation {
    pub application_association_arn: String,
    pub application_arn: String,
    pub application_id: String,
    pub client_id: String,
}
